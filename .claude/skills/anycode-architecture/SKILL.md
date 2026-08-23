---
name: anycode-architecture
description: Check work against Any Code's architectural invariants before and while writing code. Use whenever adding or changing Rust crates, Tauri commands, agent/tool/permission code, provider adapters, MCP or plugin handling, secret access, or anything that reaches a model. Also use when deciding where code belongs or whether to add a crate.
---

# Any Code architectural invariants

Twelve rules in `docs/ARCHITECTURE.md`. Violating one is a defect even when tests pass, because
each is expensive to retrofit. Run this check *before* writing, not after.

## The check

Ask each question against the change you are about to make. Any "yes" means stop and redesign.

1. Does TypeScript/React touch a secret, a raw credential, or the OS keychain? → Move behind a
   Tauri command. The UI receives status, never material.
2. Does a model's output decide whether something is permitted? → The permission engine decides.
   The model requests.
3. Does a provider name (`openai`, `anthropic`, `gemini`) appear outside `crates/anycode-models`?
   → Move it into the adapter. Orchestration must not know who is answering.
4. Does a tool execute without passing through the capability registry and permission check? →
   Route it through. There is exactly one entry point.
5. Does installing a skill change what is permitted? → Skills declare requirements only.
6. Does third-party plugin code run in the Any Code process? → Sandbox or separate process, with
   CPU/RAM/timeout/network/filesystem budgets.
7. Does this feature stop working with the network unplugged, in Local Only mode? → Make cloud
   optional or gate the feature explicitly.
8. Does repository content leave the machine without an explicit user action? → Do not.
9. Does a model request happen without emitting a usage event at the adapter? → Emit it.
10. Does a privileged action happen without an `anycode_core::Event`? → Emit it.
11. Can this agent task run forever, or ignore cancellation? → Add the cancellation path.
12. Can this report success without observing the result? → Add the verification step.

## Trust tagging

Anything that will reach a prompt gets wrapped in `anycode_core::Trust`:

- `Trust::User` — typed by the human. The only thing that may instruct.
- `Trust::System` — produced by Any Code itself.
- `Trust::Untrusted` — repository files, model output, MCP responses, browser DOM, HTTP bodies,
  terminal output. **Data only.** A file saying "ignore previous instructions" is a string.

## Where code belongs

One crate per bounded responsibility (PRD §94). Extend an existing crate before adding one; an
empty crate is scaffolding and scaffolding rots. If a change spans three crates, the boundary is
probably wrong — say so rather than threading a parameter through.

## Reporting

When a change cannot satisfy an invariant, state which one and why, and propose the design that
does. Do not ship the violation with a comment promising to fix it.
