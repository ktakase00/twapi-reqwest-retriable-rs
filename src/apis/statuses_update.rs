use crate::{error::RetriableError, LogParams, Retriable};
use twapi_reqwest::serde_json::Value;

impl Retriable {
    pub async fn statuses_update(
        &self,
        params: &Vec<(&str, &str)>,
        retry_count: usize,
        log: &impl Fn(LogParams),
    ) -> Result<Value, RetriableError> {
        let path = "https://api.twitter.com/1.1/statuses/update.json";
        let log_params = LogParams::new(path, &vec![]);
        let p1 = vec![];
        Ok(self
            .execute(retry_count, None, log_params, &vec![], log, || {
                twapi_reqwest::v1::post(
                    &path,
                    &p1,
                    &params,
                    &self.consumer_key,
                    &self.consumer_secret,
                    &self.access_key,
                    &self.access_secret,
                    self.timeout_sec,
                )
            })
            .await?
            .result)
    }
}
