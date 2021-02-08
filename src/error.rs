use thiserror::Error;

#[derive(Debug, Error)]
pub enum RetriableError {
    #[error("Twitter failed: {0}, {1}")]
    Twitter(twapi_reqwest::serde_json::Value, u16),

    #[error("Twitter Response failed: {0}, {1}")]
    TwitterResponse(String, u16),

    #[error("Twitter Media failed: {0}")]
    TwitterMedia(twapi_reqwest::serde_json::Value),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    Reqwest(#[from] twapi_reqwest::reqwest::Error),
}
