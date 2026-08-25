import { invoke } from "@tauri-apps/api/core";

/** Typed wrappers around every Rust command — the one place their names and shapes live. */

export interface WorkspaceInfo {
  path: string;
  name: string;
}

export interface FsEntry {
  name: string;
  path: string;
  isDir: boolean;
}

export type GitFileStatus =
  "modified" | "added" | "deleted" | "renamed" | "untracked" | "conflicted";

export interface GitStatusEntry {
  path: string;
  status: GitFileStatus;
}

export interface FileDiff {
  headContent: string | null;
  workingContent: string | null;
}

export const commands = {
  getTheme: () => invoke<string>("get_theme"),
  setTheme: (theme: string) => invoke<void>("set_theme", { theme }),

  getLastWorkspace: () => invoke<WorkspaceInfo | null>("get_last_workspace"),
  openWorkspace: (path: string) => invoke<WorkspaceInfo>("open_workspace", { path }),

  listDir: (relative: string) => invoke<FsEntry[]>("list_dir", { relative }),
  readFile: (relative: string) => invoke<string>("read_file", { relative }),
  writeFile: (relative: string, contents: string) =>
    invoke<void>("write_file", { relative, contents }),

  gitStatus: () => invoke<GitStatusEntry[]>("git_status"),
  gitDiff: (relative: string) => invoke<FileDiff>("git_diff", { relative }),
  gitBranch: () => invoke<string | null>("git_branch"),

  terminalSpawn: (cols: number, rows: number) => invoke<string>("terminal_spawn", { cols, rows }),
  terminalWrite: (id: string, data: string) => invoke<void>("terminal_write", { id, data }),
  terminalResize: (id: string, cols: number, rows: number) =>
    invoke<void>("terminal_resize", { id, cols, rows }),
  terminalKill: (id: string) => invoke<void>("terminal_kill", { id }),
};

export interface ProviderStatus {
  id: string;
  name: string;
  requiresKey: boolean;
  hasKey: boolean;
}

export interface ModelDefinition {
  id: string;
  displayName: string;
}

export type ChatRole = "system" | "user" | "assistant";

export interface ChatMessage {
  role: ChatRole;
  content: string;
}

export const providerCommands = {
  listProviders: () => invoke<ProviderStatus[]>("list_providers"),
  setProviderKey: (provider: string, key: string) =>
    invoke<void>("set_provider_key", { provider, key }),
  removeProviderKey: (provider: string) => invoke<void>("remove_provider_key", { provider }),
  listModels: (provider: string) => invoke<ModelDefinition[]>("list_models", { provider }),
  sendChat: (provider: string, model: string, sessionId: string, messages: ChatMessage[]) =>
    invoke<string>("send_chat", { provider, model, sessionId, messages }),
};
