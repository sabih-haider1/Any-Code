//! Tauri command boundary. Anything privileged — the filesystem, the local store —
//! lives here, never in the renderer (docs/ARCHITECTURE.md invariant #1).

use anycode_store::Store;
use std::sync::Mutex;
use tauri::{Manager, State};

struct AppState {
    store: Mutex<Store>,
}

const THEME_KEY: &str = "theme";
const DEFAULT_THEME: &str = "system";

#[tauri::command]
fn get_theme(state: State<AppState>) -> Result<String, String> {
    let store = state.store.lock().map_err(|e| e.to_string())?;
    store
        .get_setting(THEME_KEY)
        .map(|value| value.unwrap_or_else(|| DEFAULT_THEME.to_string()))
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn set_theme(state: State<AppState>, theme: String) -> Result<(), String> {
    let store = state.store.lock().map_err(|e| e.to_string())?;
    store.set_setting(THEME_KEY, &theme).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let data_dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&data_dir)?;
            let store = Store::open(data_dir.join("anycode.db"))
                .map_err(|e| format!("failed to open local store: {e}"))?;
            app.manage(AppState {
                store: Mutex::new(store),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_theme, set_theme])
        .run(tauri::generate_context!())
        .expect("error while running Any Code");
}
