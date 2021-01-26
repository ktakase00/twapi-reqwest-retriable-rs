use crate::{error::RetriableError, LogParams, Retriable};
use std::{
    fs::File,
    io::{Cursor, Read},
};
use twapi_reqwest::{
    reqwest::multipart::{Form, Part},
    serde_json::Value,
};

impl Retriable {
    pub async fn post_media_upload_append(
        &self,
        media_id: &str,
        segment_index: u64,
        file: &mut File,
        chunk_size: u64,
        retry_count: usize,
        log: &impl Fn(LogParams),
    ) -> Result<Value, RetriableError> {
        let path = "https://upload.twitter.com/1.1/media/upload.json";

        let mut buf = Vec::<u8>::with_capacity(chunk_size as usize);
        unsafe {
            buf.set_len(buf.capacity());
        };
        file.read_exact(buf.as_mut_slice())?;
        let buf = buf;
        let log_params = LogParams::new(&path, &vec![]);
        let p1 = vec![];
        Ok(self
            .execute(retry_count, None, log_params, &vec![], log, || {
                let cursor = Cursor::new(buf.clone());
                let form = Form::new()
                    .text("command", "APPEND")
                    .text("media_id", media_id.to_owned())
                    .text("segment_index", segment_index.to_string())
                    .part("media", Part::bytes(cursor.into_inner()));
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
