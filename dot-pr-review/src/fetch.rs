use crate::error::FetchError;
use octocrab::models::pulls::PullRequest;

pub trait Fetcher {
    fn fetch_pr(&self, id: u64) -> Result<PullRequest, FetchError>;
}
