use anyhow::Result;
use chrono::NaiveDate;
use clap::Parser;
use octocrab::Octocrab;

#[derive(Parser, Debug)]
#[command(name = "dot-pr-review")]
#[command(about = "List PRs created by or assigned to you, filtered by updated date")]
struct Args {
    /// Updated on or after this date (YYYY-MM-DD)
    #[arg(long, value_name = "YYYY-MM-DD")]
    updated_since: String,

    /// Optional repo filter: owner/name (e.g. rust-lang/rust)
    #[arg(long)]
    repo: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Parse date
    let date = NaiveDate::parse_from_str(&args.updated_since, "%Y-%m-%d")
        .map_err(|e| anyhow::anyhow!("invalid date '{}': {}", args.updated_since, e))?;

    // Turn into ISO date string (GitHub search only needs the date part)
    let date_str = date.format("%Y-%m-%d").to_string();

    // Build octocrab with token from env (e.g. GITHUB_TOKEN or PERSONAL_TOKEN)
    let token =
        match std::env::var("GITHUB_TOKEN").or_else(|_| std::env::var("GITHUB_PERSONAL_TOKEN")) {
            Ok(t) => t,
            Err(_) => {
                eprintln!(
                    "Error: please set GITHUB_TOKEN or GITHUB_PERSONAL_TOKEN in the environment."
                );
                eprintln!("Error: https://github.com/settings/tokens/new");
                std::process::exit(1);
            }
        };

    let octo = Octocrab::builder().personal_token(token).build()?;
    let me = octo.current().user().await?;
    println!("Logged in as {}", me.login);

    // Build the GitHub search query:
    //
    //  is:pr involves:@me updated:>=YYYY-MM-DD
    //
    let mut query = format!("is:pr involves:@me updated:>={}", date_str);

    if let Some(repo) = &args.repo {
        query.push(' ');
        query.push_str(&format!("repo:{}", repo));
    }

    // Page through results in case there are many
    let mut page = octo
        .search()
        .issues_and_pull_requests(&query)
        .send()
        .await?;

    println!("Query: {query}");
    println!("Total results (approx): {:?}", page.total_count);
    println!();

    loop {
        for item in &page.items {
            // `item` is an Issue-like object, but `pull_request` is Some for PRs.
            let author = &item.user;
            let assginee = &item.assignee.as_ref().unwrap_or(&author);
            if assginee.login != me.login {
                continue;
            }

            let title = &item.title;
            if title.contains("Backport #") && author.login.starts_with("paritytech-cmd-bot") {
                continue;
            }

            let number = &item.number;
            let updated_at = &item.updated_at;
            let state = &item.state;
            let url = &item.html_url;
            let desc = &item.body.as_deref().unwrap_or_default();

            println!("#{number}  {title} (updated: {updated_at})");
            println!(
                "    created by {}, assigned to {}",
                author.login, assginee.login
            );
            println!("    [{state:?}] {url}");
            if !desc.is_empty() {
                println!("{desc}");
            }
            println!();
        }

        if let Some(next) = octo
            .get_page::<octocrab::models::issues::Issue>(&page.next)
            .await?
        {
            page = next;
        } else {
            break;
        }
    }

    Ok(())
}
