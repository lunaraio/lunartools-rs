mod error;
mod models;

pub use error::{LunarToolsError, Result};
pub use models::*;

use serde::Serialize;
use serde_json::Value;
use std::time::Duration;

pub const DEFAULT_BASE_URL: &str = "https://remote.lunaraio.com";
pub const DEFAULT_WEBHOOK_BASE_URL: &str = "https://www.lunartools.co/api/webhooks";
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(150);

#[derive(Debug, Clone)]
pub struct LunarTools {
    base_url: String,
    webhook_base_url: String,
    http: reqwest::Client,
}

#[derive(Debug, Clone)]
pub struct LunarToolsBuilder {
    base_url: String,
    webhook_base_url: String,
    timeout: Duration,
    http: Option<reqwest::Client>,
}

impl LunarToolsBuilder {
    pub fn base_url(mut self, value: impl Into<String>) -> Self {
        self.base_url = value.into();
        self
    }

    pub fn webhook_base_url(mut self, value: impl Into<String>) -> Self {
        self.webhook_base_url = value.into();
        self
    }

    pub fn timeout(mut self, value: Duration) -> Self {
        self.timeout = value;
        self
    }

    pub fn http_client(mut self, value: reqwest::Client) -> Self {
        self.http = Some(value);
        self
    }

    pub fn build(self) -> Result<LunarTools> {
        let http = match self.http {
            Some(client) => client,
            None => reqwest::Client::builder()
                .timeout(self.timeout)
                .build()
                .map_err(|e| LunarToolsError::Internal(e.to_string()))?,
        };

        Ok(LunarTools {
            base_url: self.base_url.trim_end_matches('/').to_string(),
            webhook_base_url: self.webhook_base_url.trim_end_matches('/').to_string(),
            http,
        })
    }
}

#[derive(Serialize)]
struct Envelope<'a, T: Serialize> {
    api_key: &'a str,
    #[serde(flatten)]
    options: &'a T,
}

impl LunarTools {
    pub fn new() -> Result<Self> {
        Self::builder().build()
    }

    pub fn builder() -> LunarToolsBuilder {
        LunarToolsBuilder {
            base_url: DEFAULT_BASE_URL.to_string(),
            webhook_base_url: DEFAULT_WEBHOOK_BASE_URL.to_string(),
            timeout: DEFAULT_TIMEOUT,
            http: None,
        }
    }

    pub async fn solve(&self, api_key: &str, options: &SolveOptions) -> Result<SolveResult> {
        require("api_key", api_key)?;
        require("captcha_type", &options.captcha_type)?;
        require("page_url", &options.page_url)?;
        require("site_key", &options.site_key)?;

        self.post(
            &format!("{}/solve", self.base_url),
            &Envelope { api_key, options },
        )
        .await
    }

    pub async fn otp(&self, api_key: &str, options: &OtpOptions) -> Result<OtpResult> {
        require("api_key", api_key)?;
        require("email", &options.email)?;

        self.post(
            &format!("{}/imap", self.base_url),
            &Envelope { api_key, options },
        )
        .await
    }

    pub async fn count(&self, api_key: &str, options: &CountOptions) -> Result<CountResult> {
        require("api_key", api_key)?;
        require("email", &options.email)?;
        require("subject", &options.subject)?;

        self.post(
            &format!("{}/imap", self.base_url),
            &Envelope { api_key, options },
        )
        .await
    }

    pub async fn webhook(&self, token: &str, payload: &WebhookPayload) -> Result<WebhookResult> {
        require("token", token)?;
        if payload.is_empty() {
            return Err(LunarToolsError::BadRequest(
                "payload needs content or at least one embed".into(),
            ));
        }

        let encoded: String = token
            .bytes()
            .map(|b| match b {
                b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                    (b as char).to_string()
                }
                _ => format!("%{:02X}", b),
            })
            .collect();

        self.post(&format!("{}/{}", self.webhook_base_url, encoded), payload)
            .await
    }

    async fn post<B, T>(&self, url: &str, body: &B) -> Result<T>
    where
        B: Serialize,
        T: serde::de::DeserializeOwned,
    {
        let response = self
            .http
            .post(url)
            .json(body)
            .send()
            .await
            .map_err(|e| LunarToolsError::Network(e.to_string()))?;

        let status = response.status();
        let raw = response
            .text()
            .await
            .map_err(|e| LunarToolsError::Network(e.to_string()))?;

        if !status.is_success() {
            let parsed: Value = serde_json::from_str(&raw).unwrap_or(Value::Null);
            let code = parsed
                .get("code")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_string();
            let message = parsed
                .get("message")
                .and_then(Value::as_str)
                .or_else(|| parsed.get("error").and_then(Value::as_str))
                .unwrap_or_else(|| raw.trim())
                .to_string();

            return Err(LunarToolsError::Api {
                code,
                message,
                status: status.as_u16(),
            });
        }

        serde_json::from_str(&raw).map_err(|e| LunarToolsError::Internal(e.to_string()))
    }
}

fn require(name: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(LunarToolsError::BadRequest(format!("{} is required", name)));
    }
    Ok(())
}
