use crate::{
    error::RetriableError,
    Retriable,
    LogParams,
};
use twapi_reqwest::serde_json::Value;
use std::future::Future;

impl<U, F> Retriable<U, F>
where
    U: Future<Output = ()>,
    F: Fn(LogParams) -> U,
{
    pub async fn statuses_show(
        &self,
        twid: &str,
        retry_count: usize,
        retriable_status_codes: &Vec<u16>
    ) -> Result<Value, RetriableError> {
        let path = "https://api.twitter.com/1.1/statuses/show.json";
        let params = vec![("id", twid), ("tweet_mode", "extended"), ("include_card_uri", "true")];
        let log_params = LogParams::new(path, &params);
        Ok(self
            .execute(
                retry_count,
                None,
                log_params,
                retriable_status_codes,
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