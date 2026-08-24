//! OpenAI adapter — Chat Completions API. BYOK only (PRD §18): official ChatGPT sign-in
//! isn't something a third-party app can embed, so this speaks the plain API-key path.

use crate::provider::{ModelProvider, ModelStream};
use crate::sse::SseDecoder;
use crate::types::{
    ModelDefinition, ModelRequest, ProviderAuthMode, ProviderError, ProviderManifest, Role,
    StreamEvent, Usage,
};
use async_stream::try_stream;
use async_trait::async_trait;
use futures_util::StreamExt;
use serde_json::{json, Value};

const BASE_URL: &str = "https://api.openai.com/v1";

pub struct OpenAiProvider {
    api_key: String,
    client: reqwest::Client,
}

impl OpenAiProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
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
    let mut body = json!({
        "model": request.model,
        "messages": messages,
        "stream": true,
        "stream_options": { "include_usage": true },
    });
    if let Some(temperature) = request.temperature {
        body["temperature"] = json!(temperature);
    }
    body
}

/// One decoded SSE `data:` payload -> zero or more normalized events. OpenAI sends a
/// terminal literal `[DONE]` rather than a JSON object.
fn parse_chunk(data: &str) -> Result<Vec<StreamEvent>, ProviderError> {
    if data == "[DONE]" {
        return Ok(vec![]);
    }
    let value: Value =
        serde_json::from_str(data).map_err(|e| ProviderError::Parse(e.to_string()))?;
    let mut events = Vec::new();

    if let Some(text) = value["choices"][0]["delta"]["content"].as_str() {
        events.push(StreamEvent::TextDelta {
            text: text.to_string(),
        });
    }
    if let Some(usage) = value.get("usage").filter(|u| !u.is_null()) {
        events.push(StreamEvent::Done {
            usage: Usage {
                input_tokens: usage["prompt_tokens"].as_u64().map(|n| n as u32),
                output_tokens: usage["completion_tokens"].as_u64().map(|n| n as u32),
            },
        });
    }
    Ok(events)
}

#[async_trait]
impl ModelProvider for OpenAiProvider {
    fn manifest(&self) -> ProviderManifest {
        ProviderManifest {
            id: "openai",
            name: "OpenAI",
            auth_modes: &[ProviderAuthMode::ApiKey],
            supports_streaming: true,
            supports_tools: true,
            supports_vision: true,
        }
    }

    async fn models(&self) -> Result<Vec<ModelDefinition>, ProviderError> {
        let response = self
            .client
            .get(format!("{BASE_URL}/models"))
            .bearer_auth(&self.api_key)
            .send()
            .await?;
        if !response.status().is_success() {
            return Err(ProviderError::Api(
                response.text().await.unwrap_or_default(),
            ));
        }
        let body: Value = response.json().await?;
        let list = body["data"]
            .as_array()
            .ok_or_else(|| ProviderError::Parse("missing data array".into()))?;
        Ok(list
            .iter()
            .filter_map(|m| m["id"].as_str())
            .map(|id| ModelDefinition {
                id: id.to_string(),
                display_name: id.to_string(),
            })
            .collect())
    }

    async fn stream(&self, request: ModelRequest) -> Result<ModelStream, ProviderError> {
        let body = build_chat_request(&request);
        let response = self
            .client
            .post(format!("{BASE_URL}/chat/completions"))
            .bearer_auth(&self.api_key)
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
            let mut decoder = SseDecoder::new();
            while let Some(chunk) = bytes.next().await {
                let chunk = chunk?;
                let text = String::from_utf8_lossy(&chunk);
                for event in decoder.push(&text) {
                    for parsed in parse_chunk(&event.data)? {
                        yield parsed;
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
    use crate::types::{Message, RequestMetadata};

    #[test]
    fn builds_a_streaming_chat_request() {
        let request = ModelRequest {
            model: "gpt-5".into(),
            messages: vec![Message {
                role: Role::User,
                content: "hi".into(),
            }],
            temperature: Some(0.5),
            metadata: RequestMetadata::default(),
        };
        let body = build_chat_request(&request);
        assert_eq!(body["model"], "gpt-5");
        assert_eq!(body["messages"][0]["role"], "user");
        assert_eq!(body["stream"], true);
        assert_eq!(body["temperature"], 0.5);
    }

    #[test]
    fn parses_a_text_delta() {
        let events = parse_chunk(r#"{"choices":[{"delta":{"content":"hi"}}]}"#).unwrap();
        assert_eq!(events, vec![StreamEvent::TextDelta { text: "hi".into() }]);
    }

    #[test]
    fn parses_final_usage_chunk() {
        let events =
            parse_chunk(r#"{"choices":[],"usage":{"prompt_tokens":10,"completion_tokens":3}}"#)
                .unwrap();
        assert_eq!(
            events,
            vec![StreamEvent::Done {
                usage: Usage {
                    input_tokens: Some(10),
                    output_tokens: Some(3)
                }
            }]
        );
    }

    #[test]
    fn done_marker_yields_no_events() {
        assert_eq!(parse_chunk("[DONE]").unwrap(), vec![]);
    }
}
