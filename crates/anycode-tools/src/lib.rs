//! The capability registry (PRD §41-42): every native tool an agent can call lives here,
//! behind one trait. docs/ARCHITECTURE.md invariant #4 — "agent never bypasses the
//! capability runtime" — means exactly this: an agent that wants to touch the
//! filesystem, git, or a shell has no path to do so except through a `Tool` looked up in
//! a `ToolRegistry`. There is no other function anywhere that lets a model-originated
//! request reach the filesystem directly.
//!
//! This crate computes risk (via anycode-security) but does not decide allow/ask/deny
//! and does not check standing grants — that needs the workspace path and the local
//! store, which belong to the orchestration layer (src-tauri), not here. A tool answers
//! two questions: "what's the risk of running with this input?" and "run it."

mod filesystem;
mod git;
mod shell;

use anycode_security::RiskLevel;
use async_trait::async_trait;
use serde_json::Value;
use std::path::PathBuf;

pub use filesystem::{FilesystemReadTool, FilesystemWriteTool};
pub use git::GitStatusTool;
pub use shell::ShellExecuteTool;

#[derive(Debug, thiserror::Error)]
pub enum ToolError {
    #[error(transparent)]
    Fs(#[from] anycode_fs::FsError),
    #[error(transparent)]
    Git(#[from] anycode_git::GitError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("command timed out after {0} seconds")]
    Timeout(u64),
    #[error("invalid input: {0}")]
    InvalidInput(String),
}

/// What a tool needs to run — nothing more. No secrets, no store handle: a tool cannot
/// reach further than the workspace it was given.
pub struct ToolContext {
    pub fs_root: anycode_fs::WorkspaceRoot,
    pub workspace_path: PathBuf,
}

#[async_trait]
pub trait Tool: Send + Sync {
    /// Stable identifier, e.g. `filesystem.write.workspace`. Doubles as the capability
    /// id anycode-security's static risk table and the permission-grant store key on.
    fn name(&self) -> &'static str;

    /// Risk for *this* invocation. Most tools return a static risk via
    /// `anycode_security::capability_risk(self.name())`; shell.execute is the one
    /// exception — its risk depends on the command text, not the tool identity.
    fn risk(&self, input: &Value) -> RiskLevel;

    async fn execute(&self, input: Value, ctx: &ToolContext) -> Result<Value, ToolError>;
}

/// Every tool an agent may call, looked up by name. Construction is the one place that
/// decides what's on the menu at all — a tool not registered here cannot be invoked, no
/// matter what a model asks for.
pub struct ToolRegistry {
    tools: Vec<Box<dyn Tool>>,
}

impl ToolRegistry {
    /// The standard set available to every workspace in Phase 3.
    pub fn standard() -> Self {
        Self {
            tools: vec![
                Box::new(FilesystemReadTool),
                Box::new(FilesystemWriteTool),
                Box::new(GitStatusTool),
                Box::new(ShellExecuteTool),
            ],
        }
    }

    pub fn get(&self, name: &str) -> Option<&dyn Tool> {
        self.tools
            .iter()
            .find(|t| t.name() == name)
            .map(|t| t.as_ref())
    }

    pub fn names(&self) -> impl Iterator<Item = &'static str> + '_ {
        self.tools.iter().map(|t| t.name())
    }
}

/// Zero-dependency temp dir shared by every tool's tests, matching the pattern
/// anycode-fs and anycode-git already use.
#[cfg(test)]
pub(crate) mod test_support {
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicU64, Ordering};

    pub struct TempDir(PathBuf);
    static COUNTER: AtomicU64 = AtomicU64::new(0);

    impl TempDir {
        pub fn new() -> Self {
            let n = COUNTER.fetch_add(1, Ordering::Relaxed);
            let path =
                std::env::temp_dir().join(format!("anycode-tools-test-{}-{n}", std::process::id()));
            std::fs::create_dir_all(&path).unwrap();
            Self(path)
        }

        pub fn path(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_registry_exposes_the_expected_tools() {
        let registry = ToolRegistry::standard();
        let names: Vec<_> = registry.names().collect();
        assert!(names.contains(&"filesystem.read.workspace"));
        assert!(names.contains(&"filesystem.write.workspace"));
        assert!(names.contains(&"git.status"));
        assert!(names.contains(&"shell.execute"));
    }

    #[test]
    fn unknown_tool_name_is_not_found() {
        assert!(ToolRegistry::standard().get("database.drop").is_none());
    }
}
