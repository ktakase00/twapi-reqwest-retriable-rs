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
    pub async fn retweet(
        &self,
        twid: &str,
        retry_count: usize,
    ) -> Result<Value, RetriableError> {
        let path = format!("https://api.twitter.com/1.1/statuses/retweet/{}.json", twid);
        let log_params = LogParams::new(&path, &vec![]);
        let p1 = vec![];
        let p2 = vec![];
        Ok(self
            .execute(
                retry_count,
                None,
                log_params,
                &vec![],
                || {
                    twapi_reqwest::v1::post(
                        &path,
                        &p1,
                        &p2,
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