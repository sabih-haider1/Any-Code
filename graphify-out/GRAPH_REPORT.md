# Graph Report - Any Code  (2026-08-24)

## Corpus Check
- 110 files · ~214,602 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 993 nodes · 1371 edges · 93 communities (76 shown, 17 thin omitted)
- Extraction: 100% EXTRACTED · 0% INFERRED · 0% AMBIGUOUS · INFERRED: 4 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `22c23d8a`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- PRD.md
- README.md
- What You Must Do When Invoked
- What You Must Do When Invoked
- What You Must Do When Invoked
- event.rs
- anthropic.rs
- 98. Implementation Phases
- graphify reference: extra exports and benchmark
- graphify reference: extra exports and benchmark
- graphify reference: extra exports and benchmark
- Engineering standards
- Project rules and governance
- tauri.conf.json
- 8. Product Principles
- Adding a model provider
- graphify reference: query, path, explain
- Any Code architectural invariants
- graphify reference: query, path, explain
- graphify reference: query, path, explain
- 6. Target Users
- Definition of done
- 1. Record architecture decisions
- 2. Append-only event log with untyped payloads
- graphify reference: add a URL and watch a folder
- graphify reference: commit hook and native AGENTS.md integration
- graphify reference: incremental update and cluster-only
- graphify reference: add a URL and watch a folder
- graphify reference: commit hook and native CLAUDE.md integration
- graphify reference: incremental update and cluster-only
- graphify reference: add a URL and watch a folder
- graphify reference: commit hook and native CLAUDE.md integration
- graphify reference: incremental update and cluster-only
- 3. Positioning
- 95. Testing Strategy
- 9. Product Scope
- graphify reference: GitHub clone and cross-repo merge
- graphify reference: transcribe video and audio
- graphify reference: GitHub clone and cross-repo merge
- graphify reference: transcribe video and audio
- graphify reference: GitHub clone and cross-repo merge
- graphify reference: transcribe video and audio
- 26. Model Router
- 64. Theme System
- dependencies
- .agents/skills/graphify/references/extraction-spec.md
- .claude/CLAUDE.md
- .claude/skills/graphify/references/extraction-spec.md
- .codex/skills/graphify/references/extraction-spec.md
- 12. Technology Stack
- 4. Brand Identity
- 5. Founder Signature and Product Mark
- Any Code
- anycode-core
- compilerOptions
- Product scope
- package.json
- design-tokens/package.json
- Architecture
- App.tsx
- anycode-git/src/lib.rs
- anycode-desktop
- Releasing the desktop app
- Security model
- QA and review ledger
- Any Code — agent operating instructions
- Staging operations and monitoring
- Any Code brand
- Repository knowledge graph
- Roadmap
- AppState
- anycode-fs/src/lib.rs
- provider_commands.rs
- PtySession
- default.json
- .system
- AGENTS.md
- .prettierrc.json
- ollama.rs
- .push
- types.rs
- OpenAiProvider
- entry
- OllamaProvider
- ProviderError
- openai.rs

## God Nodes (most connected - your core abstractions)
1. `AppState` - 28 edges
2. `useWorkbenchStore` - 20 edges
3. `compilerOptions` - 15 edges
4. `ProviderError` - 13 edges
5. `ModelRequest` - 12 edges
6. `PtySession` - 12 edges
7. `What You Must Do When Invoked` - 12 edges
8. `What You Must Do When Invoked` - 12 edges
9. `What You Must Do When Invoked` - 12 edges
10. `98. Implementation Phases` - 12 edges

## Surprising Connections (you probably didn't know these)
- `anycode-desktop` --depends_on--> `anycode-fs`  [EXTRACTED]
  apps/desktop/src-tauri/Cargo.toml → crates/anycode-fs/Cargo.toml
- `anycode-desktop` --depends_on--> `anycode-git`  [EXTRACTED]
  apps/desktop/src-tauri/Cargo.toml → crates/anycode-git/Cargo.toml
- `anycode-desktop` --depends_on--> `anycode-models`  [EXTRACTED]
  apps/desktop/src-tauri/Cargo.toml → crates/anycode-models/Cargo.toml
- `anycode-desktop` --depends_on--> `anycode-secrets`  [EXTRACTED]
  apps/desktop/src-tauri/Cargo.toml → crates/anycode-secrets/Cargo.toml
- `anycode-desktop` --depends_on--> `anycode-store`  [EXTRACTED]
  apps/desktop/src-tauri/Cargo.toml → crates/anycode-store/Cargo.toml

## Import Cycles
- None detected.

## Communities (93 total, 17 thin omitted)

### Community 0 - "PRD.md"
Cohesion: 0.02
Nodes (94): 100. Quality Bar, 101. Definition of Done, 102. Repository Implementation Contract, 103. Architectural Rules, 104. Critical Product Differentiator, 105. Product North Star, 10. Non Goals for V1, 11. Platform Architecture (+86 more)

### Community 1 - "README.md"
Cohesion: 0.20
Nodes (9): Agent context, Current visual identity, Develop locally, How it works, Project status, Releases, Repository layout, Security and operations (+1 more)

### Community 2 - "What You Must Do When Invoked"
Cohesion: 0.08
Nodes (24): For /graphify add and --watch, For /graphify query, For the commit hook and native AGENTS.md integration, For --update and --cluster-only, /graphify, Honesty Rules, Interpreter guard for subcommands, Part A - Structural extraction for code files (+16 more)

### Community 3 - "What You Must Do When Invoked"
Cohesion: 0.08
Nodes (24): For /graphify add and --watch, For /graphify query, For the commit hook and native CLAUDE.md integration, For --update and --cluster-only, /graphify, Honesty Rules, Interpreter guard for subcommands, Part A - Structural extraction for code files (+16 more)

### Community 4 - "What You Must Do When Invoked"
Cohesion: 0.08
Nodes (24): For /graphify add and --watch, For /graphify query, For the commit hook and native CLAUDE.md integration, For --update and --cluster-only, /graphify, Honesty Rules, Interpreter guard for subcommands, Part A - Structural extraction for code files (+16 more)

### Community 5 - "event.rs"
Cohesion: 0.10
Nodes (27): Connection, Event, EventScope, omits_empty_scope_and_payload(), roundtrips_through_json(), Into, Option, Self (+19 more)

### Community 6 - "anthropic.rs"
Cohesion: 0.15
Nodes (14): AnthropicProvider, build_messages_request(), parse_event(), parses_content_block_delta(), parses_output_usage_from_message_delta(), Client, ModelStream, Option (+6 more)

### Community 7 - "98. Implementation Phases"
Cohesion: 0.17
Nodes (12): 98. Implementation Phases, Phase 0: Foundation, Phase 10: Mobile, Phase 1: Workbench, Phase 2: AI Provider Layer, Phase 3: Agent Runtime, Phase 4: Code Intelligence, Phase 5: Multi Agent (+4 more)

### Community 8 - "graphify reference: extra exports and benchmark"
Cohesion: 0.22
Nodes (8): graphify reference: extra exports and benchmark, Step 6b - Wiki (only if --wiki flag), Step 7 - Neo4j export (only if --neo4j or --neo4j-push flag), Step 7a - FalkorDB export (only if --falkordb or --falkordb-push flag), Step 7b - SVG export (only if --svg flag), Step 7c - GraphML export (only if --graphml flag), Step 7d - MCP server (only if --mcp flag), Step 8 - Token reduction benchmark (only if total_words > 5000)

### Community 9 - "graphify reference: extra exports and benchmark"
Cohesion: 0.22
Nodes (8): graphify reference: extra exports and benchmark, Step 6b - Wiki (only if --wiki flag), Step 7 - Neo4j export (only if --neo4j or --neo4j-push flag), Step 7a - FalkorDB export (only if --falkordb or --falkordb-push flag), Step 7b - SVG export (only if --svg flag), Step 7c - GraphML export (only if --graphml flag), Step 7d - MCP server (only if --mcp flag), Step 8 - Token reduction benchmark (only if total_words > 5000)

### Community 10 - "graphify reference: extra exports and benchmark"
Cohesion: 0.22
Nodes (8): graphify reference: extra exports and benchmark, Step 6b - Wiki (only if --wiki flag), Step 7 - Neo4j export (only if --neo4j or --neo4j-push flag), Step 7a - FalkorDB export (only if --falkordb or --falkordb-push flag), Step 7b - SVG export (only if --svg flag), Step 7c - GraphML export (only if --graphml flag), Step 7d - MCP server (only if --mcp flag), Step 8 - Token reduction benchmark (only if total_words > 5000)

### Community 11 - "Engineering standards"
Cohesion: 0.25
Nodes (8): Commits and branches, Definition of done, Dependencies, Engineering standards, Performance targets, Quality bar, Release artifacts, Testing

### Community 12 - "Project rules and governance"
Cohesion: 0.25
Nodes (8): Architecture rules, Authority and source order, Decision rules, Delivery rules, Product rules, Project rules and governance, Quality and release rules, Security and privacy rules

### Community 13 - "tauri.conf.json"
Cohesion: 0.06
Nodes (30): app, security, windows, build, beforeBuildCommand, beforeDevCommand, devUrl, frontendDist (+22 more)

### Community 14 - "8. Product Principles"
Cohesion: 0.25
Nodes (8): 8.1 Provider independence, 8.2 Local first, 8.3 Cloud optional, 8.4 Explainable routing, 8.5 Human authority, 8.6 Evidence before completion, 8.7 No artificial provider lock in, 8. Product Principles

### Community 15 - "Adding a model provider"
Cohesion: 0.29
Nodes (6): Adding a model provider, Capabilities are declared, not assumed, Contract, Local providers, Required before merge, Rules

### Community 16 - "graphify reference: query, path, explain"
Cohesion: 0.33
Nodes (5): For /graphify explain, For /graphify path, graphify reference: query, path, explain, Step 0 — Constrained query expansion (REQUIRED before traversal), Step 1 — Traversal

### Community 17 - "Any Code architectural invariants"
Cohesion: 0.33
Nodes (5): Any Code architectural invariants, Reporting, The check, Trust tagging, Where code belongs

### Community 18 - "graphify reference: query, path, explain"
Cohesion: 0.33
Nodes (5): For /graphify explain, For /graphify path, graphify reference: query, path, explain, Step 0 — Constrained query expansion (REQUIRED before traversal), Step 1 — Traversal

### Community 19 - "graphify reference: query, path, explain"
Cohesion: 0.33
Nodes (5): For /graphify explain, For /graphify path, graphify reference: query, path, explain, Step 0 — Constrained query expansion (REQUIRED before traversal), Step 1 — Traversal

### Community 20 - "6. Target Users"
Cohesion: 0.33
Nodes (6): 6.1 Individual Developer, 6.2 AI Power User, 6.3 Professional Software Engineer, 6.4 Development Team, 6.5 Enterprise, 6. Target Users

### Community 21 - "Definition of done"
Cohesion: 0.40
Nodes (4): Definition of done, Evidence, Gate, Quality bar — automatic rejection

### Community 22 - "1. Record architecture decisions"
Cohesion: 0.40
Nodes (4): 1. Record architecture decisions, Consequences, Context, Decision

### Community 23 - "2. Append-only event log with untyped payloads"
Cohesion: 0.40
Nodes (4): 2. Append-only event log with untyped payloads, Consequences, Context, Decision

### Community 24 - "graphify reference: add a URL and watch a folder"
Cohesion: 0.50
Nodes (3): For /graphify add, For --watch, graphify reference: add a URL and watch a folder

### Community 25 - "graphify reference: commit hook and native AGENTS.md integration"
Cohesion: 0.50
Nodes (3): For git commit hook, For native AGENTS.md integration, graphify reference: commit hook and native AGENTS.md integration

### Community 26 - "graphify reference: incremental update and cluster-only"
Cohesion: 0.50
Nodes (3): For --cluster-only, For --update (incremental re-extraction), graphify reference: incremental update and cluster-only

### Community 27 - "graphify reference: add a URL and watch a folder"
Cohesion: 0.50
Nodes (3): For /graphify add, For --watch, graphify reference: add a URL and watch a folder

### Community 28 - "graphify reference: commit hook and native CLAUDE.md integration"
Cohesion: 0.50
Nodes (3): For git commit hook, For native CLAUDE.md integration, graphify reference: commit hook and native CLAUDE.md integration

### Community 29 - "graphify reference: incremental update and cluster-only"
Cohesion: 0.50
Nodes (3): For --cluster-only, For --update (incremental re-extraction), graphify reference: incremental update and cluster-only

### Community 30 - "graphify reference: add a URL and watch a folder"
Cohesion: 0.50
Nodes (3): For /graphify add, For --watch, graphify reference: add a URL and watch a folder

### Community 31 - "graphify reference: commit hook and native CLAUDE.md integration"
Cohesion: 0.50
Nodes (3): For git commit hook, For native CLAUDE.md integration, graphify reference: commit hook and native CLAUDE.md integration

### Community 32 - "graphify reference: incremental update and cluster-only"
Cohesion: 0.50
Nodes (3): For --cluster-only, For --update (incremental re-extraction), graphify reference: incremental update and cluster-only

### Community 33 - "3. Positioning"
Cohesion: 0.50
Nodes (4): 3.1 Product category, 3.2 Core promise, 3.3 Secondary message, 3. Positioning

### Community 34 - "95. Testing Strategy"
Cohesion: 0.50
Nodes (4): 95. Testing Strategy, End to end tests, Integration tests, Unit tests

### Community 35 - "9. Product Scope"
Cohesion: 0.50
Nodes (4): 9.1 V1, 9.2 V1.5, 9.3 V2, 9. Product Scope

### Community 42 - "26. Model Router"
Cohesion: 0.67
Nodes (3): 26.1 Routing inputs, 26.2 Example, 26. Model Router

### Community 43 - "64. Theme System"
Cohesion: 0.67
Nodes (3): 64.1 Dark palette, 64.2 Light palette, 64. Theme System

### Community 44 - "dependencies"
Cohesion: 0.04
Nodes (46): @anycode/design-tokens, dependencies, @anycode/design-tokens, monaco-editor, react, react-dom, @tanstack/react-query, @tauri-apps/api (+38 more)

### Community 54 - "compilerOptions"
Cohesion: 0.10
Nodes (20): compilerOptions, isolatedModules, jsx, lib, module, moduleResolution, noEmit, noFallthroughCasesInSwitch (+12 more)

### Community 55 - "Product scope"
Cohesion: 0.29
Nodes (7): Change control, Product scope, Release boundaries, Scope hierarchy, Scope test for every task, V1 non-goals, V1 product contract

### Community 56 - "package.json"
Cohesion: 0.11
Nodes (18): description, devDependencies, prettier, engines, node, pnpm, homepage, name (+10 more)

### Community 57 - "design-tokens/package.json"
Cohesion: 0.22
Nodes (8): description, exports, ./tokens.css, main, name, private, type, version

### Community 58 - "Architecture"
Cohesion: 0.25
Nodes (8): Architecture, Boundaries between crates, Decisions, Event log, Invariants, Shape, Trust boundary, Verification

### Community 59 - "App.tsx"
Cohesion: 0.06
Nodes (43): App(), THEMES, Command, CommandPalette(), DiffPane(), EditorArea(), MonacoPane(), Explorer() (+35 more)

### Community 61 - "anycode-git/src/lib.rs"
Cohesion: 0.17
Nodes (21): current_branch(), diff_file(), diff_reports_head_and_working_content(), empty_repository_has_no_branch(), FileDiff, FileStatus, GitError, reports_untracked_and_modified_files() (+13 more)

### Community 64 - "anycode-desktop"
Cohesion: 0.29
Nodes (7): anycode-desktop, anycode-fs, anycode-git, anycode-models, anycode-secrets, anycode-store, anycode-terminal

### Community 65 - "Releasing the desktop app"
Cohesion: 0.29
Nodes (6): Auto-update signature (separate from code signing), macOS — code signing + notarization, Releasing the desktop app, Trigger it, What "just build it" gets you today, Windows — code signing

### Community 66 - "Security model"
Cohesion: 0.25
Nodes (7): Permissions, Reporting, Secrets, Security model, Shell risk classes, The one rule, Threat model

### Community 67 - "QA and review ledger"
Cohesion: 0.22
Nodes (9): 2026-08-23T00:41:22Z — Brand integration baseline, 2026-08-23T00:45:09Z — GitHub CI after branding and governance push, 2026-08-24 — Phase 1 workbench review and UX remediation, 2026-08-24 — Second-pass UI/UX and ledger audit, Current quality status, Open QA risks, QA and review ledger, Review protocol (+1 more)

### Community 69 - "Any Code — agent operating instructions"
Cohesion: 0.33
Nodes (6): Any Code — agent operating instructions, Before writing code, Commands, Definition of done, graphify, Non-negotiable

### Community 70 - "Staging operations and monitoring"
Cohesion: 0.33
Nodes (6): Expected staging services, Incident response, Minimum online checks, Monitoring rules, Operating boundary, Staging operations and monitoring

### Community 71 - "Any Code brand"
Cohesion: 0.40
Nodes (4): Accessibility, Any Code brand, Attribution, Canonical assets

### Community 72 - "Repository knowledge graph"
Cohesion: 0.40
Nodes (5): Installation for a new workstation, Installed integration, Repository knowledge graph, Required agent workflow, What the graph may contain

### Community 73 - "Roadmap"
Cohesion: 0.40
Nodes (5): Current phase: 1 · Workbench, Distribution gate, Phase 0 · Foundation, Roadmap, V1 success criterion

### Community 75 - "AppState"
Cohesion: 0.08
Nodes (51): list_dir(), read_file(), require_root(), Option, Result, State, String, Vec (+43 more)

### Community 76 - "anycode-fs/src/lib.rs"
Cohesion: 0.17
Nodes (20): Entry, FsError, list_dir(), read_file(), rejects_absolute_path(), rejects_parent_dir_traversal(), round_trips_a_file_inside_the_root(), AsRef (+12 more)

### Community 77 - "provider_commands.rs"
Cohesion: 0.25
Nodes (17): build_provider(), ChatDeltaEvent, ChatDoneEvent, ChatErrorEvent, list_models(), list_providers(), provider_error_message(), ProviderStatus (+9 more)

### Community 78 - "PtySession"
Cohesion: 0.17
Nodes (14): Child, default_shell(), PtySession, Box, Error, Path, Result, Self (+6 more)

### Community 79 - "default.json"
Cohesion: 0.22
Nodes (8): description, identifier, permissions, $schema, windows, core:default, dialog:default, main

### Community 81 - ".system"
Cohesion: 0.23
Nodes (7): Into, Self, String, Tagged, Tagged<T>, Trust, T

### Community 90 - "ollama.rs"
Cohesion: 0.18
Nodes (11): build_chat_request(), parse_line(), parses_a_content_line(), parses_the_final_usage_line(), role_str(), ModelStream, Option, Result (+3 more)

### Community 91 - ".push"
Cohesion: 0.29
Nodes (10): handles_multiple_events_in_one_chunk(), parses_a_single_data_only_event(), parses_named_events(), reassembles_an_event_split_across_two_chunks(), Option, Self, String, Vec (+2 more)

### Community 92 - "types.rs"
Cohesion: 0.35
Nodes (8): Message, ModelRequest, RequestMetadata, Option, String, Vec, StreamEvent, Usage

### Community 93 - "OpenAiProvider"
Cohesion: 0.25
Nodes (6): OpenAiProvider, Client, Self, String, ProviderAuthMode, ProviderManifest

### Community 94 - "entry"
Cohesion: 0.42
Nodes (9): delete_api_key(), entry(), get_api_key(), Error, Option, Result, String, SecretError (+1 more)

### Community 95 - "OllamaProvider"
Cohesion: 0.27
Nodes (8): OllamaProvider, Client, Self, String, ModelProvider, Send, Sync, Default

### Community 96 - "ProviderError"
Cohesion: 0.24
Nodes (7): Vec, ModelStream, Result, Vec, ModelDefinition, ProviderError, Error

### Community 97 - "openai.rs"
Cohesion: 0.36
Nodes (6): build_chat_request(), builds_a_streaming_chat_request(), parse_chunk(), parses_a_text_delta(), parses_final_usage_chunk(), Value

## Knowledge Gaps
- **474 isolated node(s):** `printWidth`, `trailingComma`, `name`, `description`, `homepage` (+469 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **17 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `AppState` connect `AppState` to `event.rs`, `PtySession`?**
  _High betweenness centrality (0.048) - this node is a cross-community bridge._
- **Why does `ProviderError` connect `ProviderError` to `openai.rs`, `anthropic.rs`, `provider_commands.rs`, `ollama.rs`, `types.rs`?**
  _High betweenness centrality (0.016) - this node is a cross-community bridge._
- **Why does `provider_error_message()` connect `provider_commands.rs` to `ProviderError`?**
  _High betweenness centrality (0.015) - this node is a cross-community bridge._
- **What connects `printWidth`, `trailingComma`, `name` to the rest of the system?**
  _474 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `PRD.md` be split into smaller, more focused modules?**
  _Cohesion score 0.021052631578947368 - nodes in this community are weakly interconnected._
- **Should `What You Must Do When Invoked` be split into smaller, more focused modules?**
  _Cohesion score 0.08 - nodes in this community are weakly interconnected._
- **Should `What You Must Do When Invoked` be split into smaller, more focused modules?**
  _Cohesion score 0.08 - nodes in this community are weakly interconnected._