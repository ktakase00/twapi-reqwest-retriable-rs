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
    pub async fn get_media_upload_status(
        &self,
        media_id: &str,
        retry_count: usize,
    ) -> Result<Value, RetriableError> {
        let path = "https://upload.twitter.com/1.1/media/upload.json";
        let params = vec![("command", "STATUS"), ("media_id", media_id)];
        let log_params = LogParams::new(path, &params);
        Ok(self
            .execute(
                retry_count,
                None,
                log_params,
                &vec![],
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