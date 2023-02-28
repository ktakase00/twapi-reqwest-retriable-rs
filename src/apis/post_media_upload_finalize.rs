use crate::{error::RetriableError, LogParams, Retriable};
use twapi_reqwest::{reqwest::multipart::Form, serde_json::Value};

impl Retriable {
    pub async fn post_media_upload_finalize(
        &self,
        media_id: &str,
        retry_count: usize,
        log: &impl Fn(LogParams),
    ) -> Result<Value, RetriableError> {
        let path = "https://upload.twitter.com/1.1/media/upload.json";

        let log_params = LogParams::new(&path, &vec![]);
        let p1 = vec![];
        Ok(self
            .execute(retry_count, None, log_params, &vec![], log, || {
                let form = Form::new()
                    .text("command", "FINALIZE")
                    .text("media_id", media_id.to_string());
                twapi_reqwest::v1::multipart(
                    &path,
                    &p1,
                    form,
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
