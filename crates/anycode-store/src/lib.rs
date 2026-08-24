//! Local SQLite store (PRD §75). Tables are added by the phase that produces the data
//! they hold — an empty table nobody writes to is not "done", it's a schema nobody
//! asked for yet. Two tables exist so far: `app_settings` (Phase 0) and `usage_events`
//! (Phase 2 — every model request emits telemetry, docs/ARCHITECTURE.md invariant #9).

use rusqlite::{params, Connection};
use serde::Serialize;
use std::path::Path;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    #[error(transparent)]
    Sqlite(#[from] rusqlite::Error),
}

pub struct Store {
    conn: Connection,
}

const MIGRATIONS: &str = "
    CREATE TABLE IF NOT EXISTS app_settings (
        key   TEXT PRIMARY KEY,
        value TEXT NOT NULL
    );
    CREATE TABLE IF NOT EXISTS usage_events (
        id             TEXT PRIMARY KEY,
        timestamp      TEXT NOT NULL,
        provider       TEXT NOT NULL,
        model          TEXT NOT NULL,
        input_tokens   INTEGER,
        output_tokens  INTEGER,
        status         TEXT NOT NULL
    );
";

impl Store {
    /// Opens (creating if needed) the SQLite database at `path` and applies migrations.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, StoreError> {
        let conn = Connection::open(path)?;
        conn.pragma_update(None, "journal_mode", "WAL")?;
        conn.execute_batch(MIGRATIONS)?;
        Ok(Self { conn })
    }

    pub fn open_in_memory() -> Result<Self, StoreError> {
        let conn = Connection::open_in_memory()?;
        conn.execute_batch(MIGRATIONS)?;
        Ok(Self { conn })
    }

    pub fn get_setting(&self, key: &str) -> Result<Option<String>, StoreError> {
        self.conn
            .query_row(
                "SELECT value FROM app_settings WHERE key = ?1",
                params![key],
                |row| row.get(0),
            )
            .map(Some)
            .or_else(|err| match err {
                rusqlite::Error::QueryReturnedNoRows => Ok(None),
                other => Err(other.into()),
            })
    }

    pub fn set_setting(&self, key: &str, value: &str) -> Result<(), StoreError> {
        self.conn.execute(
            "INSERT INTO app_settings (key, value) VALUES (?1, ?2)
             ON CONFLICT(key) DO UPDATE SET value = excluded.value",
            params![key, value],
        )?;
        Ok(())
    }

    /// Records one completed (or failed) model request. `input_tokens`/`output_tokens`
    /// are `None` when the provider's API didn't report them — never estimated.
    #[allow(clippy::too_many_arguments)]
    pub fn record_usage_event(
        &self,
        provider: &str,
        model: &str,
        input_tokens: Option<u32>,
        output_tokens: Option<u32>,
        status: UsageStatus,
    ) -> Result<(), StoreError> {
        self.conn.execute(
            "INSERT INTO usage_events (id, timestamp, provider, model, input_tokens, output_tokens, status)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                Uuid::new_v4().to_string(),
                OffsetDateTime::now_utc().format(&time::format_description::well_known::Rfc3339).unwrap(),
                provider,
                model,
                input_tokens,
                output_tokens,
                status.as_str(),
            ],
        )?;
        Ok(())
    }

    /// Most recent usage events, newest first. Feeds the Phase 8 usage dashboard;
    /// exercised today only by this crate's own round-trip test.
    pub fn list_usage_events(&self, limit: i64) -> Result<Vec<UsageRecord>, StoreError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, timestamp, provider, model, input_tokens, output_tokens, status
             FROM usage_events ORDER BY timestamp DESC LIMIT ?1",
        )?;
        let rows = stmt.query_map(params![limit], |row| {
            Ok(UsageRecord {
                id: row.get(0)?,
                timestamp: row.get(1)?,
                provider: row.get(2)?,
                model: row.get(3)?,
                input_tokens: row.get(4)?,
                output_tokens: row.get(5)?,
                status: row.get(6)?,
            })
        })?;
        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsageStatus {
    Success,
    Error,
}

impl UsageStatus {
    fn as_str(self) -> &'static str {
        match self {
            UsageStatus::Success => "success",
            UsageStatus::Error => "error",
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UsageRecord {
    pub id: String,
    pub timestamp: String,
    pub provider: String,
    pub model: String,
    pub input_tokens: Option<u32>,
    pub output_tokens: Option<u32>,
    pub status: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn setting_survives_get_after_set() {
        let store = Store::open_in_memory().unwrap();
        assert_eq!(store.get_setting("theme").unwrap(), None);
        store.set_setting("theme", "dark").unwrap();
        assert_eq!(store.get_setting("theme").unwrap(), Some("dark".into()));
        store.set_setting("theme", "light").unwrap();
        assert_eq!(store.get_setting("theme").unwrap(), Some("light".into()));
    }

    #[test]
    fn usage_events_round_trip_newest_first() {
        let store = Store::open_in_memory().unwrap();
        store
            .record_usage_event("openai", "gpt-5", Some(10), Some(4), UsageStatus::Success)
            .unwrap();
        store
            .record_usage_event("anthropic", "claude-opus-5", None, None, UsageStatus::Error)
            .unwrap();

        let events = store.list_usage_events(10).unwrap();
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].provider, "anthropic");
        assert_eq!(events[0].status, "error");
        assert_eq!(events[0].input_tokens, None);
        assert_eq!(events[1].provider, "openai");
        assert_eq!(events[1].input_tokens, Some(10));
    }
}
