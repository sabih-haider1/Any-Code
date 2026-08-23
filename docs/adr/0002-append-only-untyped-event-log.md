# 2. Append-only event log with untyped payloads

- **Status:** Accepted
- **Date:** 2026-08-23

## Context

PRD §77 makes events the backbone of timeline UI, audit, replay, debugging, recovery and cloud
sync. The log is append-only and long-lived: events written by today's build will be read by builds
shipped a year from now, and by older clients that have not updated. A strongly-typed enum of every
event kind would make an unrecognised variant a hard deserialisation failure — a newer desktop
build syncing to an older one would break replay entirely.

## Decision

`Event` carries a dot-namespaced `kind: String` and an open `payload: serde_json::Value`, plus
scope identifiers. Typed constructors and accessors live in the crate owning each namespace, not in
the core type.

## Consequences

- Forward and backward compatible by construction; unknown kinds are skipped, not fatal.
- Maps directly onto one SQLite/PostgreSQL table with a text `kind` column and a JSON payload.
- Cost: payload shape is not compiler-checked. Mitigated by keeping typed constructors next to the
  code that emits each kind, so exactly one place per event kind builds the payload.
- Revisit if payload drift becomes a real source of bugs rather than a theoretical one.
