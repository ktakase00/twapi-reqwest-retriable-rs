pub mod error;

use crate::error::RetriableError;
use once_cell::sync::Lazy;
use std::{future::Future, time::Duration};
use tokio::time::delay_for;
use twapi_reqwest::reqwest::Response;
use twapi_reqwest::serde_json::Value;

const STATUS_CODE_NO_CONTENT: u16 = 204;
const STATUS_CODE_TOO_MANY_REQUESTS: u16 = 429;
const STATUS_CODE_INTERNAL_SERVER_ERROR: u16 = 500;
const STATUS_CODE_SERVICE_UNAVAILABLE: u16 = 503;
const STATUS_CODE_GATEWAY_TIMEOUT: u16 = 504;

static RETRIABLE_ERRORS: Lazy<Vec<u16>> = Lazy::new(|| {
    vec![
        STATUS_CODE_INTERNAL_SERVER_ERROR,
        STATUS_CODE_SERVICE_UNAVAILABLE,
        STATUS_CODE_GATEWAY_TIMEOUT,
    ]
});

#[derive(Clone)]
pub struct LogParams {
    pub path: String,
    pub params: Vec<(String, String)>,
    pub count: usize,
    pub result: Option<Value>,
}

impl LogParams {
    fn new(path: &str, params: &Vec<(&str, &str)>) -> Self {
        let mut dst = vec![];
        for param in params {
            dst.push((param.0.to_owned(), param.1.to_owned()));
        }
        Self {
            path: path.to_owned(),
            params: dst,
            count: 0,
            result: None,
        }
    }
}

pub struct RetriableResult {
    pub result: Value,
    pub limit: u64,
    pub remaining: u64,
    pub reset: u64,
}

pub struct Retriable<U, F>
where
    U: Future<Output = ()>,
    F: Fn(LogParams) -> U,
{
    consumer_key: String,
    consumer_secret: String,
    access_key: String,
    access_secret: String,
    log: F,
}

impl<U, F> Retriable<U, F>
where
    U: Future<Output = ()>,
    F: Fn(LogParams) -> U,
{
    pub fn new(
        consumer_key: &str,
        consumer_secret: &str,
        access_key: &str,
        access_secret: &str,
        log: F,
    ) -> Self {
        Self {
            consumer_key: consumer_key.to_owned(),
            consumer_secret: consumer_secret.to_owned(),
            access_key: access_key.to_owned(),
            access_secret: access_secret.to_owned(),
            log: log,
        }
    }

    pub async fn search(
        &self,
        params: &Vec<(&str, &str)>,
        retry_count: usize,
        retry_delay_secound_count: usize,
    ) -> Result<Value, RetriableError> {
        let path = "https://api.twitter.com/1.1/search/tweets.json";
        let log_params = LogParams::new(path, params);
        Ok(self
            .execute(
                retry_count,
                Some(retry_delay_secound_count),
                log_params,
                &RETRIABLE_ERRORS,
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

    pub async fn user_timeline(
        &self,
        params: &Vec<(&str, &str)>,
        retry_count: usize,
        retry_delay_secound_count: usize,
    ) -> Result<Value, RetriableError> {
        let path = "https://api.twitter.com/1.1/search/user_timeline.json";
        let log_params = LogParams::new(path, params);
        Ok(self
            .execute(
                retry_count,
                Some(retry_delay_secound_count),
                log_params,
                &RETRIABLE_ERRORS,
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

    async fn execute<Executor, ResponseFutuer>(
        &self,
        retry_count: usize,
        retry_delay_secound_count: Option<usize>,
        log_params: LogParams,
        retryable_status_codes: &Vec<u16>,
        executor: Executor,
    ) -> Result<RetriableResult, RetriableError>
    where
        ResponseFutuer: Future<Output = Result<Response, twapi_reqwest::reqwest::Error>>,
        Executor: Fn() -> ResponseFutuer,
    {
        // カウンター初期化
        let mut count: usize = 0;

        // エラー初期化
        let mut err: RetriableError;

        loop {
            (self.log)(log_params.clone()).await;

            let response = executor().await?;

            let status_code: u16 = response.status().as_u16();

            let limit: u64 = get_header_value(&response, "x-rate-limit-limit");
            let remaining: u64 = get_header_value(&response, "x-rate-limit-remaining");
            let reset: u64 = get_header_value(&response, "x-rate-limit-reset");

            let text = if status_code == STATUS_CODE_NO_CONTENT {
                "{}".to_owned()
            } else {
                response.text().await.unwrap_or("text not found".to_owned())
            };
            let json: Result<Value, twapi_reqwest::serde_json::Error> =
                twapi_reqwest::serde_json::from_str(&text);

            match json {
                Ok(json) => {
                    let mut log_params2 = log_params.clone();
                    log_params2.result = Some(json.clone());
                    (self.log)(log_params2);

                    // エラー判定。status_codeはとりあえず放置
                    if json["errors"].is_array() || json["error"].is_string() {
                        err = RetriableError::Twitter(json, status_code);
                        if !retryable_status_codes.contains(&status_code) {
                            return Err(err);
                        }
                    } else {
                        // 成功
                        return Ok(RetriableResult {
                            result: json,
                            limit: limit,
                            remaining: remaining,
                            reset: reset,
                        });
                    }
                }
                // JSONの変換に失敗
                Err(_) => {
                    err = RetriableError::TwitterResponse(text, status_code);
                    // ステータスコードがRate Limitなら復旧する見込みは無いので直ぐに終了する
                    if status_code == STATUS_CODE_TOO_MANY_REQUESTS {
                        return Err(err);
                    }
                }
            }

            // リトライ数チェック
            if count >= retry_count {
                return Err(err);
            }
            count = count + 1;

            // スリープ
            match retry_delay_secound_count {
                // 固定値でスリープ
                Some(retry_delay_secound_count) => sleep(retry_delay_secound_count as u64).await,

                // リトライ間隔を開けてスリープ
                None => sleep(2_i64.pow(count as u32) as u64).await,
            }
        }
    }
}

fn get_header_value(response: &Response, key: &str) -> u64 {
    match response.headers().get(key) {
        Some(value) => value.to_str().unwrap_or("0").parse().unwrap_or(0),
        None => 0,
    }
}

async fn sleep(seconds: u64) {
    delay_for(Duration::from_secs(seconds)).await;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
