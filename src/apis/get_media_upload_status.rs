use crate::{error::RetriableError, LogParams, Retriable};
use twapi_reqwest::serde_json::Value;

impl Retriable {
    pub async fn get_media_upload_status(
        &self,
        media_id: &str,
        retry_count: usize,
        log: &impl Fn(LogParams),
    ) -> Result<Value, RetriableError> {
        let path = "https://upload.twitter.com/1.1/media/upload.json";
        let params = vec![("command", "STATUS"), ("media_id", media_id)];
        let log_params = LogParams::new(path, &params);
        Ok(self
            .execute(retry_count, None, log_params, &vec![], log, || {
                twapi_reqwest::v1::get(
                    &path,
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
