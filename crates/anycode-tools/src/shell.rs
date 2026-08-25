use crate::{Tool, ToolContext, ToolError};
use anycode_security::{classify_shell_command, RiskLevel};
use async_trait::async_trait;
use serde_json::{json, Value};
use std::time::Duration;
use tokio::process::Command;
use tokio::time::timeout;

const DEFAULT_TIMEOUT_SECS: u64 = 30;

/// One-shot command execution with captured output — distinct from the interactive PTY
/// sessions the Terminal panel uses (Phase 1's `anycode-terminal`). An agent doesn't want
/// a shell to type into; it wants a command's stdout, stderr, and exit code.
pub struct ShellExecuteTool;

#[async_trait]
impl Tool for ShellExecuteTool {
    fn name(&self) -> &'static str {
        "shell.execute"
    }

    fn risk(&self, input: &Value) -> RiskLevel {
        match input["command"].as_str() {
            Some(command) => classify_shell_command(command),
            // Malformed input has no command text to classify — refuse to guess low.
            None => RiskLevel::Critical,
        }
    }

    fn description(&self) -> &'static str {
        "Run a shell command in the workspace root and capture its stdout, stderr, and exit code."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "command": { "type": "string", "description": "The command to run, e.g. \"npm test\"." },
            },
            "required": ["command"],
        })
    }

    async fn execute(&self, input: Value, ctx: &ToolContext) -> Result<Value, ToolError> {
        let command = input["command"]
            .as_str()
            .ok_or_else(|| ToolError::InvalidInput("missing \"command\"".into()))?;

        let (shell, flag) = if cfg!(windows) {
            ("cmd", "/C")
        } else {
            ("sh", "-c")
        };
        let run = Command::new(shell)
            .arg(flag)
            .arg(command)
            .current_dir(&ctx.workspace_path)
            .output();

        let output = timeout(Duration::from_secs(DEFAULT_TIMEOUT_SECS), run)
            .await
            .map_err(|_| ToolError::Timeout(DEFAULT_TIMEOUT_SECS))??;

        Ok(json!({
            "stdout": String::from_utf8_lossy(&output.stdout),
            "stderr": String::from_utf8_lossy(&output.stderr),
            "exitCode": output.status.code(),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::TempDir;

    fn context(dir: &TempDir) -> ToolContext {
        let fs_root = anycode_fs::WorkspaceRoot::new(dir.path()).unwrap();
        ToolContext {
            fs_root,
            workspace_path: dir.path().to_path_buf(),
        }
    }

    #[tokio::test]
    async fn captures_stdout_and_exit_code() {
        let dir = TempDir::new();
        let result = ShellExecuteTool
            .execute(json!({ "command": "echo hi" }), &context(&dir))
            .await
            .unwrap();
        assert_eq!(result["stdout"].as_str().unwrap().trim(), "hi");
        assert_eq!(result["exitCode"], 0);
    }

    #[tokio::test]
    async fn a_failing_command_is_a_result_not_an_error() {
        let dir = TempDir::new();
        let result = ShellExecuteTool
            .execute(json!({ "command": "exit 7" }), &context(&dir))
            .await
            .unwrap();
        assert_eq!(result["exitCode"], 7);
    }

    #[test]
    fn missing_command_text_classifies_as_critical_not_low() {
        // Refuses to default a malformed request to a permissive risk level.
        assert_eq!(ShellExecuteTool.risk(&json!({})), RiskLevel::Critical);
    }

    #[test]
    fn risk_follows_the_command_text() {
        assert_eq!(
            ShellExecuteTool.risk(&json!({"command": "git status"})),
            RiskLevel::Low
        );
        assert_eq!(
            ShellExecuteTool.risk(&json!({"command": "rm -rf /"})),
            RiskLevel::Critical
        );
    }
}
