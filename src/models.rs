use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize)]
pub struct SolveOptions {
    pub captcha_type: String,
    pub page_url: String,
    pub site_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_ms: Option<u64>,
}

impl SolveOptions {
    pub fn new(
        captcha_type: impl Into<String>,
        page_url: impl Into<String>,
        site_key: impl Into<String>,
    ) -> Self {
        Self {
            captcha_type: captcha_type.into(),
            page_url: page_url.into(),
            site_key: site_key.into(),
            proxy_url: None,
            timeout_ms: None,
        }
    }

    pub fn proxy_url(mut self, value: impl Into<String>) -> Self {
        self.proxy_url = Some(value.into());
        self
    }

    pub fn timeout_ms(mut self, value: u64) -> Self {
        self.timeout_ms = Some(value);
        self
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SolveResult {
    #[serde(default)]
    pub token: String,
    #[serde(default)]
    pub solve_ms: u64,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct OtpOptions {
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imap_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_ms: Option<u64>,
}

impl OtpOptions {
    pub fn new(email: impl Into<String>) -> Self {
        Self {
            email: email.into(),
            ..Default::default()
        }
    }

    pub fn imap_email(mut self, value: impl Into<String>) -> Self {
        self.imap_email = Some(value.into());
        self
    }

    pub fn from(mut self, value: impl Into<String>) -> Self {
        self.from = Some(value.into());
        self
    }

    pub fn site(mut self, value: impl Into<String>) -> Self {
        self.site = Some(value.into());
        self
    }

    pub fn regex(mut self, value: impl Into<String>) -> Self {
        self.regex = Some(value.into());
        self
    }

    pub fn timeout_ms(mut self, value: u64) -> Self {
        self.timeout_ms = Some(value);
        self
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct OtpResult {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub otp_code: String,
    #[serde(default)]
    pub imap_email: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CountOptions {
    pub email: String,
    pub subject: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imap_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
}

impl CountOptions {
    pub fn new(email: impl Into<String>, subject: impl Into<String>) -> Self {
        Self {
            email: email.into(),
            subject: subject.into(),
            ..Default::default()
        }
    }

    pub fn imap_email(mut self, value: impl Into<String>) -> Self {
        self.imap_email = Some(value.into());
        self
    }

    pub fn from(mut self, value: impl Into<String>) -> Self {
        self.from = Some(value.into());
        self
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CountResult {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub count: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct EmbedField {
    pub name: String,
    pub value: String,
    #[serde(skip_serializing_if = "is_false")]
    pub inline: bool,
}

impl EmbedField {
    pub fn new(name: impl Into<String>, value: impl Into<String>, inline: bool) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
            inline,
        }
    }
}

fn is_false(value: &bool) -> bool {
    !*value
}

#[derive(Debug, Clone, Serialize)]
pub struct EmbedFooter {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct EmbedAuthor {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct EmbedMedia {
    pub url: String,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct Embed {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<EmbedFooter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<EmbedMedia>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<EmbedMedia>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<EmbedAuthor>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<EmbedField>,
}

impl Embed {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn color(mut self, value: u32) -> Self {
        self.color = Some(value);
        self
    }

    pub fn field(mut self, name: impl Into<String>, value: impl Into<String>, inline: bool) -> Self {
        self.fields.push(EmbedField::new(name, value, inline));
        self
    }
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct WebhookPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub embeds: Vec<Embed>,
}

impl WebhookPayload {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());
        self
    }

    pub fn avatar_url(mut self, value: impl Into<String>) -> Self {
        self.avatar_url = Some(value.into());
        self
    }

    pub fn embed(mut self, embed: Embed) -> Self {
        self.embeds.push(embed);
        self
    }

    pub fn is_empty(&self) -> bool {
        self.content.as_deref().unwrap_or("").is_empty() && self.embeds.is_empty()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct WebhookResult {
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub count: u64,
    #[serde(default)]
    pub delivered: u64,
}
