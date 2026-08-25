use crate::{Tool, ToolContext, ToolError};
use anycode_security::{capability_risk, RiskLevel};
use async_trait::async_trait;
use serde_json::{json, Value};

pub struct FilesystemReadTool;

#[async_trait]
impl Tool for FilesystemReadTool {
    fn name(&self) -> &'static str {
        "filesystem.read.workspace"
    }

    fn risk(&self, _input: &Value) -> RiskLevel {
        capability_risk(self.name())
    }

    fn description(&self) -> &'static str {
        "Read a text file's full contents from the open workspace."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "path": { "type": "string", "description": "Path relative to the workspace root." },
            },
            "required": ["path"],
        })
    }

    async fn execute(&self, input: Value, ctx: &ToolContext) -> Result<Value, ToolError> {
        let path = input["path"]
            .as_str()
            .ok_or_else(|| ToolError::InvalidInput("missing \"path\"".into()))?;
        let content = anycode_fs::read_file(&ctx.fs_root, path)?;
        Ok(json!({ "content": content }))
    }
}

pub struct FilesystemWriteTool;

#[async_trait]
impl Tool for FilesystemWriteTool {
    fn name(&self) -> &'static str {
        "filesystem.write.workspace"
    }

    fn risk(&self, _input: &Value) -> RiskLevel {
        capability_risk(self.name())
    }

    fn description(&self) -> &'static str {
        "Write (creating or overwriting) a text file in the open workspace."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "path": { "type": "string", "description": "Path relative to the workspace root." },
                "content": { "type": "string", "description": "Full file contents to write." },
            },
            "required": ["path", "content"],
        })
    }

    async fn execute(&self, input: Value, ctx: &ToolContext) -> Result<Value, ToolError> {
        let path = input["path"]
            .as_str()
            .ok_or_else(|| ToolError::InvalidInput("missing \"path\"".into()))?;
        let content = input["content"]
            .as_str()
            .ok_or_else(|| ToolError::InvalidInput("missing \"content\"".into()))?;
        anycode_fs::write_file(&ctx.fs_root, path, content)?;
        Ok(json!({}))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::TempDir;
    use std::path::Path;

    fn context() -> (TempDir, ToolContext) {
        let dir = TempDir::new();
        let fs_root = anycode_fs::WorkspaceRoot::new(dir.path()).unwrap();
        let workspace_path = dir.path().to_path_buf();
        (
            dir,
            ToolContext {
                fs_root,
                workspace_path,
            },
        )
    }

    #[tokio::test]
    async fn write_then_read_round_trips() {
        let (_dir, ctx) = context();
        FilesystemWriteTool
            .execute(json!({ "path": "a.txt", "content": "hello" }), &ctx)
            .await
            .unwrap();
        let result = FilesystemReadTool
            .execute(json!({ "path": "a.txt" }), &ctx)
            .await
            .unwrap();
        assert_eq!(result["content"], "hello");
    }

    #[tokio::test]
    async fn read_rejects_missing_path_argument() {
        let (_dir, ctx) = context();
        let err = FilesystemReadTool
            .execute(json!({}), &ctx)
            .await
            .unwrap_err();
        assert!(matches!(err, ToolError::InvalidInput(_)));
    }

    #[tokio::test]
    async fn write_cannot_escape_the_workspace_root() {
        let (_dir, ctx) = context();
        let err = FilesystemWriteTool
            .execute(json!({ "path": "../escape.txt", "content": "x" }), &ctx)
            .await
            .unwrap_err();
        assert!(matches!(err, ToolError::Fs(_)));
        assert!(!Path::new("/tmp/escape.txt").exists());
    }
}
