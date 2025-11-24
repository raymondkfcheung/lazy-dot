use anyhow::Result;
use chrono::NaiveDate;
use clap::Parser;
use dot_pr_review::fetch::Fetcher;

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

    // Build the GitHub search query:
    //
    //  is:pr involves:@me updated:>=YYYY-MM-DD
    //
    let mut query = format!("involves:@me updated:>={}", date_str);

    if let Some(repo) = &args.repo {
        query.push(' ');
        query.push_str(&format!("repo:{}", repo));
    }

    // Page through results in case there are many
    let fectcher = Fetcher::new()?;
    let me = fectcher.current_user().await?;
    let mut page = fectcher.fetch(&query).await?;

    loop {
        for item in &page.items {
            // `item` is an Issue-like object, but `pull_request` is Some for PRs.
            let is_pr = item.pull_request.is_some();
            let author = &item.user;
            let assginee = &item.assignee.as_ref().unwrap_or(&author);
            if assginee.login != me.login {
                continue;
            }

            let title = &item.title;
            if title.contains("Backport #")
                && author.login.starts_with("paritytech-")
                && author.login.ends_with("[bot]")
            {
                continue;
            }

            let number = &item.number;
            let updated_at = &item.updated_at;
            let state = &item.state;
            let url = &item.html_url;
            let desc = &item.body.as_deref().unwrap_or_default();

            println!(
                "{}#{number}  {title} (updated: {updated_at})",
                if is_pr { "PR" } else { "Issue" }
            );
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

        if let Some(next) = fectcher.fetch_next(&page).await? {
            page = next;
        } else {
            break;
        }
    }

    Ok(())
}
