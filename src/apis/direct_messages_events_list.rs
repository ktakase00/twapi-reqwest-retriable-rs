use crate::{error::RetriableError, LogParams, Retriable, RetriableResult, RETRIABLE_ERRORS};

impl Retriable {
    pub async fn direct_messages_events_list(
        &self,
        params: &Vec<(&str, &str)>,
        retry_count: usize,
        retry_delay_secound_count: usize,
        log: &impl Fn(LogParams),
    ) -> Result<RetriableResult, RetriableError> {
        let path = "https://api.twitter.com/1.1/direct_messages/events/list.json";
        let log_params = LogParams::new(path, params);
        self.execute(
            retry_count,
            Some(retry_delay_secound_count),
            log_params,
            &RETRIABLE_ERRORS,
            log,
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
        .await
    }
}
