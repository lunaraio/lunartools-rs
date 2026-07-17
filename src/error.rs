use thiserror::Error;

#[derive(Debug, Error)]
pub enum LunarToolsError {
    #[error("{code}: {message}")]
    Api {
        code: String,
        message: String,
        status: u16,
    },

    #[error("bad_request: {0}")]
    BadRequest(String),

    #[error("network: {0}")]
    Network(String),

    #[error("internal: {0}")]
    Internal(String),
}

impl LunarToolsError {
    pub fn code(&self) -> &str {
        match self {
            Self::Api { code, .. } => code,
            Self::BadRequest(_) => "bad_request",
            Self::Network(_) => "network",
            Self::Internal(_) => "internal",
        }
    }

    pub fn status(&self) -> u16 {
        match self {
            Self::Api { status, .. } => *status,
            _ => 0,
        }
    }

    pub fn retryable(&self) -> bool {
        matches!(
            self.code(),
            "client_offline" | "client_disconnected" | "auth_unavailable" | "too_many_inflight"
        )
    }
}

pub type Result<T> = std::result::Result<T, LunarToolsError>;
