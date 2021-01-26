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
    pub async fn statuses_update(
        &self,
        params: &Vec<(&str, &str)>,
        retry_count: usize,
    ) -> Result<Value, RetriableError> {
        let path = "https://api.twitter.com/1.1/statuses/update.json";
        let log_params = LogParams::new(path, &vec![]);
        let p1 = vec![];
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