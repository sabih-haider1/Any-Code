//! Append-only event log primitives.
//!
//! `kind` is a dot-namespaced string (`task.started`, `model.request.failed`) and
//! `payload` is free-form JSON. This is deliberate: the log is append-only and must
//! stay readable by older and newer builds alike, so an unrecognised `kind` is skipped
//! rather than failing the whole replay. Typed accessors belong in the crates that own
//! each namespace, not here.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use time::OffsetDateTime;
use uuid::Uuid;

/// What an event is attached to. Every event has a session; the rest narrow it down.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventScope {
    pub session_id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<Uuid>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Event {
    pub id: Uuid,
    #[serde(with = "time::serde::rfc3339")]
    pub timestamp: OffsetDateTime,
    /// Dot-namespaced kind, e.g. `agent.started`. See PRD §77.
    pub kind: String,
    #[serde(flatten)]
    pub scope: EventScope,
    #[serde(default, skip_serializing_if = "Value::is_null")]
    pub payload: Value,
}

impl Event {
    pub fn new(kind: impl Into<String>, scope: EventScope, payload: Value) -> Self {
        Self {
            id: Uuid::new_v4(),
            timestamp: OffsetDateTime::now_utc(),
            kind: kind.into(),
            scope,
            payload,
        }
    }

    /// `agent.started` -> `agent`. Used for filtering and routing.
    pub fn namespace(&self) -> &str {
        self.kind.split('.').next().unwrap_or("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scope() -> EventScope {
        EventScope {
            session_id: Uuid::new_v4(),
            ..Default::default()
        }
    }

    #[test]
    fn roundtrips_through_json() {
        let event = Event::new("task.started", scope(), serde_json::json!({"prompt": "hi"}));
        let text = serde_json::to_string(&event).unwrap();
        assert_eq!(event, serde_json::from_str::<Event>(&text).unwrap());
    }

    #[test]
    fn reads_events_written_by_a_newer_build() {
        // Forward compatibility is the whole point of the untyped payload: an
        // unfamiliar kind and unfamiliar payload fields must still deserialize.
        let text = r#"{
            "id": "0192f0b0-0000-7000-8000-000000000001",
            "timestamp": "2026-08-23T04:51:00Z",
            "kind": "quantum.teleport.completed",
            "session_id": "0192f0b0-0000-7000-8000-000000000002",
            "payload": { "unknown_field": 42 }
        }"#;
        let event: Event = serde_json::from_str(text).unwrap();
        assert_eq!(event.namespace(), "quantum");
        assert_eq!(event.payload["unknown_field"], 42);
    }

    #[test]
    fn omits_empty_scope_and_payload() {
        let text =
            serde_json::to_string(&Event::new("session.created", scope(), Value::Null)).unwrap();
        assert!(!text.contains("workspace_id"), "{text}");
        assert!(!text.contains("payload"), "{text}");
    }
}
