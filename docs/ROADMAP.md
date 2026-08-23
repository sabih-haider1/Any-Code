# Roadmap

Phases from [PRD.md](../PRD.md) §98. A phase is done when its **exit condition** is demonstrated on
a real repository — not when its files exist.

Release boundaries and rules for admitting work are defined in
[PRODUCT-SCOPE.md](PRODUCT-SCOPE.md). V1 covers Phases 0–8; V1.5 begins with Phase 9; V2 begins
with Phase 10. Only the current phase is active implementation scope.

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
- [x] Design tokens + dark/light/high-contrast themes
- [x] Tauri 2 shell + React shell
- [x] SQLite local store and migrations
- [ ] Signed builds for Windows and macOS — unsigned pipeline built (`.github/workflows/desktop-release.yml`);
      blocked on the user supplying Apple/Windows signing certificates, see [RELEASING.md](RELEASING.md)

## Distribution gate

GitHub Actions is the canonical packaging path. A platform is not considered shipped until CI can
build its native installer from a clean checkout, run the applicable tests, sign it with protected
release credentials, generate checksums, and publish it as a versioned release artifact.

Expected formats as platform clients become available:

| Platform | Required release artifacts |
|----------|----------------------------|
| macOS | Signed and notarized `.dmg`, plus the updater artifact required by Tauri |
| Windows | Signed `.exe`/NSIS installer and `.msi`, plus the updater artifact |
| Linux | `.AppImage` and `.deb`; add `.rpm` when Linux support enters active scope |
| Android | Signed `.apk` for testing and `.aab` for store distribution |
| iOS | Signed archive/`.ipa` delivered through the approved Apple distribution workflow |

Desktop packaging belongs to Phase 0. Android and iOS jobs must not be presented as supported
release jobs before Phase 10 supplies real mobile application targets. CI may use unsigned
artifacts on pull requests, but anything labeled a release must be signed and traceable to its
source commit.

## V1 success criterion

A new user installs Any Code, opens a repository, connects one provider, asks for a feature,
watches execution, reviews commands and diffs, runs tests, sees the cost, restarts the app and
resumes the session — with no configuration outside Any Code beyond provider authorisation.
