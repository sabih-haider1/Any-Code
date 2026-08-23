# Architecture

Derived from [PRD.md](../PRD.md) §11–13, §41–52, §103. This document is the enforceable subset:
every rule here is checkable in review.

## Shape

```
React + TypeScript  ──  UI only. Never privileged.
        │
     Tauri 2        ──  Command boundary. Everything privileged crosses here.
        │
   Rust runtime     ──  Agents, routing, tools, git, index, MCP, secrets, metering.
        │
  ┌─────┼─────┐
Local  Cloud  OS    ──  SQLite / optional API / keychain, PTY, filesystem.
```

## Invariants

These are the ones that are expensive to retrofit. Each is a review gate.

| # | Invariant | Why it is load-bearing |
|---|-----------|------------------------|
| 1 | UI never owns secrets | A renderer compromise must not yield credentials. Secrets live in OS keychain, reachable only from Rust. |
| 2 | LLM never owns permissions | The runtime decides what executes. A model can *request*; it can never *authorise*. |
| 3 | Provider adapters never own orchestration | Swapping OpenAI for a local model must not touch the agent loop. Model-specific code stops at the adapter boundary. |
| 4 | Agents never bypass the capability runtime | Native tools, MCP, connectors and plugins enter through one registry with one permission check. |
| 5 | Skills never grant permissions | A skill declares requirements; the permission engine decides. Otherwise installing a skill is privilege escalation. |
| 6 | Plugins never run in the main process | Third-party code crashes and misbehaves. It gets a sandbox and a resource budget. |
| 7 | Cloud is never required for Local Only | Every local feature must work with the network unplugged. |
| 8 | Repository indexing stays local by default | Uploading source is an explicit user choice, never a side effect. |
| 9 | Every expensive model request emits a usage event | Cost visibility cannot be bolted on afterwards; the event is emitted at the adapter, not the call site. |
| 10 | Every privileged action is auditable | One event stream feeds the timeline, the audit log and sync. |
| 11 | Every agent task is cancellable | Unbounded execution is a defect, not a performance issue. |
| 12 | Every automated completion is verifiable | See Verification below. |

## Trust boundary

Three sources, one rule: only the user instructs.

```
user input        → Trust::User        → may instruct
Any Code itself   → Trust::System      → may instruct within its own scope
everything else   → Trust::Untrusted   → data only
```

"Everything else" includes repository files, model output, MCP responses, browser DOM, HTTP bodies
and terminal output. `anycode_core::Trust` exists so this is a type, not a convention.

## Event log

Everything meaningful appends an `anycode_core::Event`. The log is the single mechanism behind
timeline UI, audit, replay, debugging, recovery and sync — so it is append-only and
forward-compatible: `kind` is a namespaced string and `payload` is open JSON. An older build must
survive reading a newer build's events by skipping what it does not recognise.

## Verification

An agent may not report success because code was written. Default gate:

```
compiles + lints + tests pass + process starts + behaviour observed + diff reviewed
```

Frontend work adds browser evidence: console clean, network sane, screenshot captured, DOM
asserted. Evidence is attached to the task; unverified work is reported as unverified.

## Boundaries between crates

One crate per bounded responsibility, named per PRD §94. A crate may not reach past its boundary:
`anycode-agent` orchestrates but does not know provider names; `anycode-models` speaks to providers
but does not plan; `anycode-security` decides permissions and is called by both.

Add a crate only when a responsibility genuinely exists. An empty crate is scaffolding, and
scaffolding rots.

## Decisions

Non-obvious architectural choices get an ADR in [adr/](adr/).
