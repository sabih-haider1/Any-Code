//! Git status/diff for the currently open workspace. Read-only (see anycode-git).

use crate::workspace::current_path;
use crate::AppState;
use anycode_git::{FileDiff, StatusEntry};
use tauri::State;

#[tauri::command]
pub fn git_status(state: State<AppState>) -> Result<Vec<StatusEntry>, String> {
    anycode_git::status(&current_path(&state)?).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn git_diff(state: State<AppState>, relative: String) -> Result<FileDiff, String> {
    anycode_git::diff_file(&current_path(&state)?, &relative).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn git_branch(state: State<AppState>) -> Result<Option<String>, String> {
    anycode_git::current_branch(&current_path(&state)?).map_err(|e| e.to_string())
}
