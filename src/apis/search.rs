use crate::{
    error::RetriableError,
    Retriable,
    LogParams,
    RETRIABLE_ERRORS,
};
use twapi_reqwest::serde_json::Value;
use std::future::Future;

impl<U, F> Retriable<U, F>
where
    U: Future<Output = ()>,
    F: Fn(LogParams) -> U,
{
    pub async fn search(
        &self,
        params: &Vec<(&str, &str)>,
        retry_count: usize,
        retry_delay_secound_count: usize,
    ) -> Result<Value, RetriableError> {
        let path = "https://api.twitter.com/1.1/search/tweets.json";
        let log_params = LogParams::new(path, params);
        Ok(self
            .execute(
                retry_count,
                Some(retry_delay_secound_count),
                log_params,
                &RETRIABLE_ERRORS,
                || {
                    twapi_reqwest::v1::get(
                        &path,
                        &params,
                        &self.consumer_key,
                        &self.consumer_secret,
                        &self.access_key,
                        &self.access_secret,
                    )
                },
            )
            .await?
            .result)
    }
}