//! Client for the Anthropic Models API (`GET /v1/models`).
//!
//! Fetches model metadata including context windows, output limits, and capability
//! flags (thinking, effort). Used by `ModelsManager` to keep the bundled catalog
//! fresh with API-reported values.

use crate::ANTHROPIC_VERSION_HEADER_VALUE;
use crate::client::AnthropicAuth;
use http::HeaderMap;
use http::HeaderValue;
use http::Method;
use orbit_code_client::HttpTransport;
use orbit_code_client::Request;
use orbit_code_client::RequestCompression;
use orbit_code_client::ReqwestTransport;
use serde::Deserialize;
use serde::Serialize;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::timeout;

/// Client for the Anthropic `/v1/models` endpoint.
pub struct AnthropicModelsClient {
    transport: Arc<dyn HttpTransport>,
    base_url: String,
    request_timeout: Duration,
}

/// A single model entry from the Anthropic `/v1/models` response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnthropicModelInfo {
    pub id: String,
    #[serde(default)]
    pub display_name: String,
    /// Maximum input tokens (context window).
    #[serde(default)]
    pub max_input_tokens: i64,
    /// Maximum output tokens the model can produce.
    #[serde(default, rename = "max_tokens")]
    pub max_output_tokens: i64,
    /// Nested capability descriptors.
    #[serde(default)]
    pub capabilities: AnthropicCapabilities,
}

/// Capability metadata returned by the Anthropic Models API.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AnthropicCapabilities {
    #[serde(default)]
    pub thinking: ThinkingCapability,
    #[serde(default)]
    pub effort: EffortCapability,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ThinkingCapability {
    #[serde(default)]
    pub supported: bool,
    #[serde(default)]
    pub types: ThinkingTypes,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ThinkingTypes {
    #[serde(default)]
    pub enabled: SupportedFlag,
    #[serde(default)]
    pub adaptive: SupportedFlag,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SupportedFlag {
    #[serde(default)]
    pub supported: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EffortCapability {
    #[serde(default)]
    pub supported: bool,
    #[serde(default)]
    pub low: SupportedFlag,
    #[serde(default)]
    pub medium: SupportedFlag,
    #[serde(default)]
    pub high: SupportedFlag,
    #[serde(default)]
    pub max: SupportedFlag,
}

/// Paginated response from `GET /v1/models`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnthropicModelsResponse {
    pub data: Vec<AnthropicModelInfo>,
    #[serde(default)]
    pub has_more: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_id: Option<String>,
}

impl AnthropicModelsClient {
    pub fn new(client: reqwest::Client, base_url: String, request_timeout: Duration) -> Self {
        Self {
            transport: Arc::new(ReqwestTransport::new(client)),
            base_url,
            request_timeout,
        }
    }

    #[cfg(test)]
    pub(crate) fn with_transport(
        transport: Arc<dyn HttpTransport>,
        base_url: String,
        request_timeout: Duration,
    ) -> Self {
        Self {
            transport,
            base_url,
            request_timeout,
        }
    }

    /// Fetch all models, handling pagination via `after_id`.
    /// Requests up to 1000 models per page (covers all current Anthropic models).
    pub async fn list_models(
        &self,
        auth: &AnthropicAuth,
    ) -> std::result::Result<Vec<AnthropicModelInfo>, AnthropicModelsError> {
        let mut all_models = Vec::new();
        let mut after_id: Option<String> = None;

        loop {
            let mut url = format!("{}/v1/models?limit=1000", self.base_url);
            if let Some(cursor) = &after_id {
                url.push_str("&after_id=");
                url.push_str(cursor);
            }

            let mut headers = HeaderMap::new();
            headers.insert(
                "anthropic-version",
                HeaderValue::from_static(ANTHROPIC_VERSION_HEADER_VALUE),
            );
            match auth {
                AnthropicAuth::ApiKey(key) => {
                    let val = HeaderValue::from_str(key)
                        .map_err(|e| AnthropicModelsError::InvalidHeader(e.to_string()))?;
                    headers.insert("x-api-key", val);
                }
                AnthropicAuth::BearerToken(token) => {
                    let val = HeaderValue::from_str(&format!("Bearer {token}"))
                        .map_err(|e| AnthropicModelsError::InvalidHeader(e.to_string()))?;
                    headers.insert("authorization", val);
                }
            }

            let request = Request {
                method: Method::GET,
                url,
                headers,
                body: None,
                compression: RequestCompression::None,
                timeout: Some(self.request_timeout),
            };

            let response = timeout(self.request_timeout, self.transport.execute(request))
                .await
                .map_err(|_| AnthropicModelsError::Timeout)?
                .map_err(AnthropicModelsError::Transport)?;

            if !response.status.is_success() {
                return Err(AnthropicModelsError::ApiError {
                    status: response.status.as_u16(),
                    body: String::from_utf8_lossy(&response.body).into_owned(),
                });
            }

            let page: AnthropicModelsResponse = serde_json::from_slice(&response.body)
                .map_err(|e| AnthropicModelsError::Parse(e.to_string()))?;

            let last_id = page.data.last().map(|m| m.id.clone());
            all_models.extend(page.data);

            if !page.has_more {
                break;
            }
            after_id = last_id;
        }

        Ok(all_models)
    }
}

/// Errors specific to the Models API (separate from the streaming Messages API errors).
#[derive(Debug, thiserror::Error)]
pub enum AnthropicModelsError {
    #[error("Anthropic models API returned {status}: {body}")]
    ApiError { status: u16, body: String },
    #[error("request timed out")]
    Timeout,
    #[error("transport error: {0}")]
    Transport(#[from] orbit_code_client::TransportError),
    #[error("failed to parse response: {0}")]
    Parse(String),
    #[error("invalid header value: {0}")]
    InvalidHeader(String),
}

impl AnthropicModelsError {
    /// Whether this error is a 401 Unauthorized (expected for OAuth).
    pub fn is_unauthorized(&self) -> bool {
        matches!(self, Self::ApiError { status: 401, .. })
    }

    /// Whether this error is retryable (5xx server errors).
    pub fn is_retryable(&self) -> bool {
        matches!(self, Self::ApiError { status, .. } if *status >= 500)
    }
}

#[cfg(test)]
#[path = "models_tests.rs"]
mod tests;
