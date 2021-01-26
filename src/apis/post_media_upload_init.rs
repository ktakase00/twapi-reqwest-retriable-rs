use crate::{error::RetriableError, LogParams, Retriable};
use twapi_reqwest::{reqwest::multipart::Form, serde_json::Value};

impl Retriable {
    pub async fn post_media_upload_init(
        &self,
        file_size: u64,
        media_type: &str,
        media_category: &str,
        additional_owners: Option<String>,
        retry_count: usize,
        log: &impl Fn(LogParams),
    ) -> Result<Value, RetriableError> {
        let path = "https://upload.twitter.com/1.1/media/upload.json";

        let log_params = LogParams::new(&path, &vec![]);
        let p1 = vec![];
        Ok(self
            .execute(retry_count, None, log_params, &vec![], log, || {
                let mut form = Form::new()
                    .text("command", "INIT")
                    .text("total_bytes", file_size.to_string())
                    .text("media_type", String::from(media_type))
                    .text("media_category", String::from(media_category));
                if let Some(additional_owners) = &additional_owners {
                    form = form.text("additional_owners", additional_owners.clone());
                }
                twapi_reqwest::v1::multipart(
                    &path,
                    &p1,
                    form,
                    &self.consumer_key,
                    &self.consumer_secret,
                    &self.access_key,
                    &self.access_secret,
                )
            })
            .await?
            .result)
    }
}
