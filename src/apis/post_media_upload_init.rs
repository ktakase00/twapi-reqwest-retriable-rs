use crate::{error::RetriableError, LogParams, Retriable};
use twapi_reqwest::serde_json::Value;

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
        Ok(self
            .post_media_upload_init_dm(
                file_size.clone(),
                media_type,
                media_category,
                additional_owners.map(|it| it.to_owned()),
                None,
                retry_count.clone(),
                log,
            )
            .await?)
    }
}
