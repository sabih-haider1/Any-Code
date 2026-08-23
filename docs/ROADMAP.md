# Roadmap

Phases from [PRD.md](../PRD.md) §98. A phase is done when its **exit condition** is demonstrated on
a real repository — not when its files exist.

Phases are sequential for a reason: each one is the foundation the next assumes. Building Phase 5
parallelism before Phase 3's approval system means shipping unbounded agents with no brakes.

| Phase | Ships | Exit condition |
|-------|-------|----------------|
| **0 · Foundation** | Monorepo, Tauri + React shell, Rust core, CI, lint/format/test, SQLite, event system, design tokens, themes | App launches on Windows and macOS |
| **1 · Workbench** | Workspace selection, explorer, Monaco, tabs, terminal, git status, diff, command palette, settings | Usable as a lightweight coding environment without AI |
| **2 · Provider layer** | Provider abstraction, OpenAI, Anthropic, Gemini, OpenRouter, Ollama, streaming, model selector, credential vault, usage events | Same task switches providers with no change outside the adapter |
| **3 · Agent runtime** | Planner, task state machine, tool calls, filesystem/terminal/git tools, approvals, timeline, verification | Agent implements *and verifies* a simple repository task |
| **4 · Code intelligence** | tree-sitter, ripgrep, LSP, symbol index, SQLite FTS, context builder, context inspector, memory | Agents retrieve targeted context instead of dumping files |
| **5 · Multi-agent** | Task DAG, subagents, git worktrees, parallel execution, merge coordinator, agent dashboard | Two agents work concurrently without corrupting the workspace |
| **6 · Browser verification** | Playwright, browser panel, screenshots, console, network, DOM, responsive checks | A frontend agent proves its work |
| **7 · Capability platform** | MCP, connectors, skills, plugin host, capability permissions, install UX | All external capabilities flow through one registry |
| **8 · Subscription intelligence** | Usage dashboard, budgets, provider/model analytics, subscription ledger, routing policies | User can see where AI money goes and change it |
| **9 · Cloud** | Identity, cloud API, PostgreSQL, sync, devices, remote sessions, web dashboard | A session continues on another authorised device |
| **10 · Mobile** | iOS, Android, task management, diff review, approvals, usage, notifications | Real work supervised without a laptop |

## Current phase: 0

- [x] Repository, git, CI, formatting, linting, testing
- [x] Event system foundation (`anycode-core`)
- [x] Trust tagging foundation (`anycode-core`)
- [x] Architecture, standards and security documents
- [ ] Design tokens + dark/light/high-contrast themes
- [ ] Tauri 2 shell + React shell
- [ ] SQLite local store and migrations
- [ ] Signed builds for Windows and macOS

## V1 success criterion

A new user installs Any Code, opens a repository, connects one provider, asks for a feature,
watches execution, reviews commands and diffs, runs tests, sees the cost, restarts the app and
resumes the session — with no configuration outside Any Code beyond provider authorisation.
