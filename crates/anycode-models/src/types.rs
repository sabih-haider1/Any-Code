//! Normalized types every provider adapter speaks (PRD §17, §79-80). Nothing here may
//! reference a specific vendor's request/response shape — that translation lives entirely
//! inside each adapter (docs/ARCHITECTURE.md invariant #3).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderAuthMode {
    OAuth,
    DeviceCode,
    ApiKey,
    AccessToken,
    ServiceAccount,
    CloudCredentials,
    Local,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderManifest {
    pub id: &'static str,
    pub name: &'static str,
    pub auth_modes: &'static [ProviderAuthMode],
    pub supports_streaming: bool,
    pub supports_tools: bool,
    pub supports_vision: bool,
}

/// A model as the provider itself reports it — id and display name only. Anything a
/// provider doesn't actually expose (context window, pricing) stays absent rather than
/// guessed; see docs/SECURITY.md and the provider-adapter skill's "never invent numbers".
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelDefinition {
    pub id: String,
    pub display_name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    System,
    User,
    Assistant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestMetadata {
    pub session_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelRequest {
    pub model: String,
    pub messages: Vec<Message>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(default)]
    pub metadata: RequestMetadata,
}

/// Token accounting for one completed request. `None` for a field the provider's API
/// genuinely doesn't report — never estimated in its place.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Usage {
    pub input_tokens: Option<u32>,
    pub output_tokens: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum StreamEvent {
    TextDelta { text: String },
    Done { usage: Usage },
}

#[derive(Debug, thiserror::Error)]
pub enum ProviderError {
    #[error("no API key configured for this provider")]
    MissingCredential,
    #[error("request failed: {0}")]
    Http(#[from] reqwest::Error),
    #[error("provider returned an error: {0}")]
    Api(String),
    #[error("could not parse provider response: {0}")]
    Parse(String),
}
