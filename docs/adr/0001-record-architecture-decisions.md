# 1. Record architecture decisions

- **Status:** Accepted
- **Date:** 2026-08-23

## Context

The PRD specifies a large system. Decisions made early — provider abstraction shape, permission
model, event log format — constrain everything after them. Without a record, the reasoning is lost
and the decision gets relitigated or silently reversed.

## Decision

Record every non-obvious architectural decision as a numbered ADR in `docs/adr/`. One file per
decision: context, decision, consequences. Superseded ADRs stay in place and link forward.

An ADR is warranted when the choice is hard to reverse, affects more than one crate, or is one a
future reader would otherwise ask "why on earth" about.

## Consequences

Small cost per decision. The alternative is rediscovering constraints by breaking them.
