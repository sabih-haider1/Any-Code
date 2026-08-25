//! The agent loop (Phase 3). Given an instruction, lets the model call tools from the
//! standard `ToolRegistry` — but every call crosses `anycode-security`'s gate first.
//! This is the only place in the application that ever calls `Tool::execute`: no other
//! code path lets a model-originated request touch the filesystem, git, or a shell
//! (docs/ARCHITECTURE.md invariants #2 and #4).

use crate::provider_commands::build_provider;
use crate::workspace::current_path;
use crate::AppState;
use anycode_models::{
    Message, ModelRequest, RequestMetadata, Role, StreamEvent, ToolCallRequest, ToolDefinition,
};
use anycode_security::{decide, Decision, StandingGrant};
use anycode_store::UsageStatus;
use anycode_tools::ToolContext;
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager};
use tokio::sync::oneshot;
use uuid::Uuid;

/// The user's answer to an `task:approval_requested` prompt. `AllowWorkspace` persists
/// a standing grant (anycode-store); `AllowOnce` only affects this one call.
#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalResponse {
    AllowOnce,
    AllowWorkspace,
    Deny,
}

/// A pending approval is abandoned rather than left open forever — an agent task
/// shouldn't hang indefinitely because a prompt was never answered.
const APPROVAL_TIMEOUT: Duration = Duration::from_secs(300);
/// Bounds a runaway tool-call loop (a model that never stops asking for tools).
const MAX_TOOL_ROUNDS: u32 = 8;

#[derive(Clone, Serialize)]
struct TaskDeltaEvent {
    text: String,
}
#[derive(Clone, Serialize)]
struct TaskToolCallEvent {
    id: String,
    name: String,
    arguments: Value,
    risk: &'static str,
}
#[derive(Clone, Serialize)]
struct TaskToolResultEvent {
    id: String,
    name: String,
    result: Value,
}
#[derive(Clone, Serialize)]
struct TaskApprovalRequestedEvent {
    id: String,
    name: String,
    arguments: Value,
    risk: &'static str,
}
#[derive(Clone, Serialize)]
struct TaskDoneEvent {
    text: String,
}
#[derive(Clone, Serialize)]
struct TaskErrorEvent {
    message: String,
}

fn risk_label(risk: anycode_security::RiskLevel) -> &'static str {
    use anycode_security::RiskLevel::*;
    match risk {
        Low => "low",
        Medium => "medium",
        High => "high",
        Critical => "critical",
    }
}

/// Starts an agent task and returns immediately with a task id. Progress arrives as
/// `task:delta:{id}` / `task:tool_call:{id}` / `task:approval_requested:{id}` /
/// `task:tool_result:{id}` / `task:done:{id}` / `task:error:{id}` events.
#[tauri::command]
pub fn run_task(
    app: AppHandle,
    provider: String,
    model: String,
    session_id: String,
    instruction: String,
) -> Result<String, String> {
    let adapter = build_provider(&provider)?;
    let workspace_path = {
        let state = app.state::<AppState>();
        current_path(&state)?
    };
    let fs_root = anycode_fs::WorkspaceRoot::new(&workspace_path).map_err(|e| e.to_string())?;

    let task_id = Uuid::new_v4().to_string();
    let emit_id = task_id.clone();

    tauri::async_runtime::spawn(async move {
        let tool_defs: Vec<ToolDefinition> = {
            let state = app.state::<AppState>();
            state
                .tools
                .specs()
                .into_iter()
                .map(|s| ToolDefinition {
                    name: s.name.to_string(),
                    description: s.description.to_string(),
                    input_schema: s.input_schema,
                })
                .collect()
        };

        let record_usage = |input: Option<u32>, output: Option<u32>, status: UsageStatus| {
            if let Some(state) = app.try_state::<AppState>() {
                if let Ok(store) = state.store.lock() {
                    let _ = store.record_usage_event(&provider, &model, input, output, status);
                }
            }
        };

        let mut messages = vec![Message::user(instruction)];
        let mut final_text = String::new();

        for _round in 0..MAX_TOOL_ROUNDS {
            let request = ModelRequest {
                model: model.clone(),
                messages: messages.clone(),
                temperature: None,
                tools: Some(tool_defs.clone()),
                metadata: RequestMetadata { session_id: session_id.clone(), task_id: Some(emit_id.clone()) },
            };

            let mut stream = match adapter.stream(request).await {
                Ok(s) => s,
                Err(err) => {
                    record_usage(None, None, UsageStatus::Error);
                    let _ = app.emit(&format!("task:error:{emit_id}"), TaskErrorEvent { message: err.to_string() });
                    return;
                }
            };

            let mut round_text = String::new();
            let mut tool_calls: Vec<ToolCallRequest> = Vec::new();

            while let Some(event) = stream.next().await {
                match event {
                    Ok(StreamEvent::TextDelta { text }) => {
                        round_text.push_str(&text);
                        let _ = app.emit(&format!("task:delta:{emit_id}"), TaskDeltaEvent { text });
                    }
                    Ok(StreamEvent::ToolCall { id, name, arguments }) => {
                        tool_calls.push(ToolCallRequest { id, name, arguments });
                    }
                    Ok(StreamEvent::Done { usage }) => {
                        record_usage(usage.input_tokens, usage.output_tokens, UsageStatus::Success);
                    }
                    Err(err) => {
                        record_usage(None, None, UsageStatus::Error);
                        let _ = app.emit(&format!("task:error:{emit_id}"), TaskErrorEvent { message: err.to_string() });
                        return;
                    }
                }
            }

            final_text.push_str(&round_text);

            if tool_calls.is_empty() {
                let _ = app.emit(&format!("task:done:{emit_id}"), TaskDoneEvent { text: final_text });
                return;
            }

            messages.push(Message {
                role: Role::Assistant,
                content: round_text,
                tool_calls: Some(tool_calls.clone()),
                tool_call_id: None,
            });

            for call in tool_calls {
                let result = run_gated_tool(&app, &workspace_path, &fs_root, &call).await;
                let _ = app.emit(
                    &format!("task:tool_result:{emit_id}"),
                    TaskToolResultEvent { id: call.id.clone(), name: call.name.clone(), result: result.clone() },
                );
                messages.push(Message {
                    role: Role::Tool,
                    content: result.to_string(),
                    tool_calls: None,
                    tool_call_id: Some(call.id),
                });
            }
        }

        let _ = app.emit(
            &format!("task:error:{emit_id}"),
            TaskErrorEvent { message: format!("stopped after {MAX_TOOL_ROUNDS} tool-call rounds without finishing") },
        );
    });

    Ok(task_id)
}

/// Runs one tool call through the permission gate: compute risk, check for a standing
/// workspace grant, decide, execute or ask or deny. Always returns a JSON value — a
/// denial or an execution failure is data the model needs to see, not a Rust error.
async fn run_gated_tool(
    app: &AppHandle,
    workspace_path: &std::path::Path,
    fs_root: &anycode_fs::WorkspaceRoot,
    call: &ToolCallRequest,
) -> Value {
    let state = app.state::<AppState>();
    let Some(tool) = state.tools.get(&call.name) else {
        return json!({ "error": format!("unknown tool: {}", call.name) });
    };

    let risk = tool.risk(&call.arguments);
    let workspace_key = workspace_path.to_string_lossy().to_string();
    let grant = match state.store.lock() {
        Ok(store) => match store.has_permission_grant(&call.name, &workspace_key) {
            Ok(true) => StandingGrant::WorkspaceAllowed,
            Ok(false) => StandingGrant::None,
            Err(_) => StandingGrant::None,
        },
        Err(_) => StandingGrant::None,
    };

    let _ = app.emit(
        &format!("task:tool_call:{}", call.id),
        TaskToolCallEvent {
            id: call.id.clone(),
            name: call.name.clone(),
            arguments: call.arguments.clone(),
            risk: risk_label(risk),
        },
    );

    let decision = decide(risk, grant);
    let approved = match decision {
        Decision::Deny => {
            return json!({ "error": "denied by workspace policy", "risk": risk_label(risk) });
        }
        Decision::Allow => true,
        Decision::Ask => match request_approval(app, call, risk).await {
            ApprovalResponse::AllowOnce => true,
            ApprovalResponse::AllowWorkspace => {
                if let Ok(store) = state.store.lock() {
                    let _ = store.grant_permission(&call.name, &workspace_key);
                }
                true
            }
            ApprovalResponse::Deny => false,
        },
    };

    if !approved {
        return json!({ "error": "denied by user" });
    }

    let ctx = ToolContext { fs_root: fs_root.clone(), workspace_path: workspace_path.to_path_buf() };
    match tool.execute(call.arguments.clone(), &ctx).await {
        Ok(value) => value,
        Err(err) => json!({ "error": err.to_string() }),
    }
}

/// Asks the frontend to approve `call` and waits for `respond_to_approval` to answer —
/// or the timeout, treated as a denial rather than leaving the task hanging forever.
async fn request_approval(
    app: &AppHandle,
    call: &ToolCallRequest,
    risk: anycode_security::RiskLevel,
) -> ApprovalResponse {
    let (tx, rx) = oneshot::channel();
    insert_pending_approval(app, call.id.clone(), tx);

    let _ = app.emit(
        &format!("task:approval_requested:{}", call.id),
        TaskApprovalRequestedEvent {
            id: call.id.clone(),
            name: call.name.clone(),
            arguments: call.arguments.clone(),
            risk: risk_label(risk),
        },
    );

    match tokio::time::timeout(APPROVAL_TIMEOUT, rx).await {
        Ok(Ok(response)) => response,
        _ => {
            remove_pending_approval(app, &call.id);
            ApprovalResponse::Deny
        }
    }
}

/// Isolated in its own function so the `State` guard and the `MutexGuard` it produces
/// both go out of scope at return, rather than interacting with a caller's own borrows.
fn insert_pending_approval(app: &AppHandle, id: String, sender: oneshot::Sender<ApprovalResponse>) {
    let state = app.state::<AppState>();
    if let Ok(mut pending) = state.pending_approvals.lock() {
        pending.insert(id, sender);
    };
}

fn remove_pending_approval(app: &AppHandle, id: &str) {
    let state = app.state::<AppState>();
    if let Ok(mut pending) = state.pending_approvals.lock() {
        pending.remove(id);
    };
}

/// Answers a pending `task:approval_requested` prompt raised by [`run_task`].
#[tauri::command]
pub fn respond_to_approval(app: AppHandle, id: String, response: ApprovalResponse) -> Result<(), String> {
    let state = app.state::<AppState>();
    let sender = state
        .pending_approvals
        .lock()
        .map_err(|e| e.to_string())?
        .remove(&id)
        .ok_or_else(|| "no pending approval with that id".to_string())?;
    sender.send(response).map_err(|_| "approval request was already abandoned".to_string())
}

