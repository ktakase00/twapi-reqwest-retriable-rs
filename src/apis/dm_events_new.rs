use crate::{error::RetriableError, LogParams, Retriable};
use twapi_reqwest::serde_json::Value;

impl Retriable {
    pub async fn dm_events_new(
        &self,
        json: &Value,
        retry_count: usize,
        log: &impl Fn(LogParams),
    ) -> Result<Value, RetriableError> {
        let path = "https://api.twitter.com/1.1/direct_messages/events/new.json";
        let log_params = LogParams::new(path, &vec![]);
        let p1 = vec![];
        Ok(self
            .execute(retry_count, None, log_params, &vec![], log, || {
                twapi_reqwest::v1::json(
                    &path,
                    &p1,
                    &json,
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
