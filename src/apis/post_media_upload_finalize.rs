use crate::{
    error::RetriableError,
    Retriable,
    LogParams,
};
use twapi_reqwest::{
    reqwest::multipart::{Form},
    serde_json::Value,
};
use std::{
    future::Future,
};

impl<U, F> Retriable<U, F>
where
    U: Future<Output = ()>,
    F: Fn(LogParams) -> U,
{
    pub async fn post_media_upload_finalize(
        &self,
        media_id: &str,
        retry_count: usize
    ) -> Result<Value, RetriableError> {
        let path = "https://upload.twitter.com/1.1/media/upload.json";

        let log_params = LogParams::new(&path, &vec![]);
        let p1 = vec![];
        Ok(self
            .execute(
                retry_count,
                None,
                log_params,
                &vec![],
                || {
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
                    )
                },
            )
            .await?
            .result)
    }
}