//! Ollama adapter — local models, no credential (PRD §21). Unlike OpenAI/Anthropic,
//! Ollama streams newline-delimited JSON objects rather than SSE, so it doesn't use
//! `sse.rs`; each line is already a complete message.

use crate::provider::{ModelProvider, ModelStream};
use crate::types::{
    ModelDefinition, ModelRequest, ProviderAuthMode, ProviderError, ProviderManifest, Role,
    StreamEvent, Usage,
};
use async_stream::try_stream;
use async_trait::async_trait;
use futures_util::StreamExt;
use serde_json::{json, Value};

const DEFAULT_BASE_URL: &str = "http://localhost:11434";

pub struct OllamaProvider {
    base_url: String,
    client: reqwest::Client,
}

impl Default for OllamaProvider {
    fn default() -> Self {
        Self::new(DEFAULT_BASE_URL.to_string())
    }
}

impl OllamaProvider {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: reqwest::Client::new(),
        }
    }
}

fn role_str(role: Role) -> &'static str {
    match role {
        Role::System => "system",
        Role::User => "user",
        Role::Assistant => "assistant",
    }
}

fn build_chat_request(request: &ModelRequest) -> Value {
    let messages: Vec<Value> = request
        .messages
        .iter()
        .map(|m| json!({ "role": role_str(m.role), "content": m.content }))
        .collect();
    let mut body = json!({ "model": request.model, "messages": messages, "stream": true });
    if let Some(temperature) = request.temperature {
        body["options"] = json!({ "temperature": temperature });
    }
    body
}

/// One newline-delimited JSON object -> zero or one normalized event.
fn parse_line(line: &str) -> Result<Option<StreamEvent>, ProviderError> {
    if line.trim().is_empty() {
        return Ok(None);
    }
    let value: Value =
        serde_json::from_str(line).map_err(|e| ProviderError::Parse(e.to_string()))?;

    if value["done"].as_bool() == Some(true) {
        return Ok(Some(StreamEvent::Done {
            usage: Usage {
                input_tokens: value["prompt_eval_count"].as_u64().map(|n| n as u32),
                output_tokens: value["eval_count"].as_u64().map(|n| n as u32),
            },
        }));
    }
    if let Some(text) = value["message"]["content"]
        .as_str()
        .filter(|t| !t.is_empty())
    {
        return Ok(Some(StreamEvent::TextDelta {
            text: text.to_string(),
        }));
    }
    Ok(None)
}

#[async_trait]
impl ModelProvider for OllamaProvider {
    fn manifest(&self) -> ProviderManifest {
        ProviderManifest {
            id: "ollama",
            name: "Ollama",
            auth_modes: &[ProviderAuthMode::Local],
            supports_streaming: true,
            supports_tools: false,
            supports_vision: false,
        }
    }

    async fn models(&self) -> Result<Vec<ModelDefinition>, ProviderError> {
        let response = self
            .client
            .get(format!("{}/api/tags", self.base_url))
            .send()
            .await?;
        if !response.status().is_success() {
            return Err(ProviderError::Api(
                response.text().await.unwrap_or_default(),
            ));
        }
        let body: Value = response.json().await?;
        let list = body["models"]
            .as_array()
            .ok_or_else(|| ProviderError::Parse("missing models array".into()))?;
        Ok(list
            .iter()
            .filter_map(|m| m["name"].as_str())
            .map(|name| ModelDefinition {
                id: name.to_string(),
                display_name: name.to_string(),
            })
            .collect())
    }

    async fn stream(&self, request: ModelRequest) -> Result<ModelStream, ProviderError> {
        let body = build_chat_request(&request);
        let response = self
            .client
            .post(format!("{}/api/chat", self.base_url))
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(ProviderError::Api(
                response.text().await.unwrap_or_default(),
            ));
        }

        let stream = try_stream! {
            let mut bytes = response.bytes_stream();
            let mut buffer = String::new();
            while let Some(chunk) = bytes.next().await {
                let chunk = chunk?;
                buffer.push_str(&String::from_utf8_lossy(&chunk));
                while let Some(pos) = buffer.find('\n') {
                    let line: String = buffer.drain(..=pos).collect();
                    if let Some(event) = parse_line(line.trim_end())? {
                        yield event;
                    }
                }
            }
        };
        Ok(Box::pin(stream))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_content_line() {
        let event = parse_line(r#"{"message":{"content":"hi"},"done":false}"#).unwrap();
        assert_eq!(event, Some(StreamEvent::TextDelta { text: "hi".into() }));
    }

    #[test]
    fn parses_the_final_usage_line() {
        let event = parse_line(r#"{"done":true,"prompt_eval_count":12,"eval_count":4}"#).unwrap();
        assert_eq!(
            event,
            Some(StreamEvent::Done {
                usage: Usage {
                    input_tokens: Some(12),
                    output_tokens: Some(4)
                }
            })
        );
    }

    #[test]
    fn ignores_blank_lines() {
        assert_eq!(parse_line("").unwrap(), None);
        assert_eq!(parse_line("   ").unwrap(), None);
    }
}
