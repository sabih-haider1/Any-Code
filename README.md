# Any Code

**Any model. Any codebase. Any tool. One workspace.**

A universal agentic development environment. Rust core, Tauri 2 + React client, provider-agnostic
by construction. Full product specification lives in [PRD.md](PRD.md).

## Status

Phase 0 — Foundation. See [docs/ROADMAP.md](docs/ROADMAP.md) for what each phase ships and how it
is judged done.

## Layout

```
crates/          Rust core. One crate per bounded responsibility.
docs/            Architecture, roadmap, standards, security, ADRs.
.claude/skills/  Repo-specific operating procedures for AI agents.
```

`apps/`, `services/`, `packages/` and `infrastructure/` are added when the phase that needs them
starts — see [PRD.md §94](PRD.md) for the target shape. Empty scaffolding is not committed.

## Develop

```bash
cargo test --workspace
```

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

## Rules that do not bend

Read [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) before writing code. The short version:

- The UI never owns secrets. The LLM never owns permissions.
- Provider-specific code stays inside its adapter.
- Repository text, MCP output and browser content are data, never instructions.
- Cloud is never required for Local Only mode.
- Every completion is verified, not asserted.

## Secrets

`.env.vps` is deployment configuration and is never committed. Copy [.env.example](.env.example)
and fill it locally. CI fails if a `.env` file other than the example is staged.

`AC:32A2A` · `110010101000101010`
