use crate::error::FetchError;
use octocrab::{
    Octocrab, Page,
    models::{Author, issues::Issue},
};

pub struct Fetcher {
    octo: Octocrab,
}

impl Fetcher {
    pub fn new() -> Result<Self, FetchError> {
        let token = Self::get_token()?;

        let octo = Octocrab::builder()
            .personal_token(token)
            .build()
            .map_err(|e| {
                eprintln!("Error: failed to connect to GitHub: {e:?}");
                FetchError::IoError("Failed to connect to GitHub".to_string())
            })?;

        Ok(Self { octo })
    }

    pub async fn current_user(&self) -> Result<Author, FetchError> {
        let me = self.octo.current().user().await.map_err(|e| {
            eprintln!("Error: failed to get user info: {e:?}");
            FetchError::NotFound("Failed to get user info".to_string())
        })?;

        println!("Logged in as {}", me.login);

        Ok(me)
    }

    pub async fn fetch(&self, query: &str) -> Result<Page<Issue>, FetchError> {
        // Page through results in case there are many
        let page = self
            .octo
            .search()
            .issues_and_pull_requests(query)
            .send()
            .await
            .map_err(|e| {
                eprintln!("Error: failed to search {query}: {e:?}");
                FetchError::IoError("Failed to search".to_string())
            })?;

        println!("Query: {query}");
        println!("Total results (approx): {:?}", page.total_count);
        println!();

        Ok(page)
    }

    pub async fn fetch_next(
        &self,
        current: &Page<Issue>,
    ) -> Result<Option<Page<Issue>>, FetchError> {
        let next_page = self
            .octo
            .get_page::<Issue>(&current.next)
            .await
            .map_err(|e| {
                eprintln!("Error: failed to get next page: {e:?}");
                FetchError::IoError("Failed to get next page".to_string())
            })?;

        Ok(next_page)
    }

    fn get_token() -> Result<String, FetchError> {
        // Get token from env (e.g. GITHUB_TOKEN or PERSONAL_TOKEN)
        return match std::env::var("GITHUB_TOKEN")
            .or_else(|_| std::env::var("GITHUB_PERSONAL_TOKEN"))
        {
            Ok(t) => Ok(t),
            Err(_) => {
                eprintln!(
                    "Error: please set GITHUB_TOKEN or GITHUB_PERSONAL_TOKEN in the environment."
                );
                eprintln!(
                    "Error: please generate a new token at https://github.com/settings/tokens/new"
                );
                Err(FetchError::MissingToken)
            }
        };
    }
}
