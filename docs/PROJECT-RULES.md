# Project rules and governance

These rules keep Any Code coherent as people and coding agents contribute to it. They are binding
for product, design, engineering, testing, documentation, and release work.

## Authority and source order

When sources conflict, use this order:

1. Applicable law, provider terms, security obligations, and user consent
2. Accepted architecture decisions in [`docs/adr/`](adr/)
3. [PRODUCT-SCOPE.md](PRODUCT-SCOPE.md) and [ARCHITECTURE.md](ARCHITECTURE.md)
4. [PRD.md](../PRD.md)
5. [ROADMAP.md](ROADMAP.md)
6. Feature notes, issue descriptions, mock-ups, and chat transcripts

The initial ChatGPT conversation is product discovery material. It explains intent and options; it
is not an implementation command and cannot override the repository's approved documents.

## Product rules

1. **Build the engineering runtime, not a multi-model chat wrapper.** Context, orchestration,
   permissions, execution, memory, usage, and verification are the product; models are replaceable.
2. **Stay provider-neutral.** No provider-specific behavior may leak beyond its adapter. Officially
   supported authentication only; never imitate, scrape, or transfer another product's subscription.
3. **Remain local-first.** Local repositories, indexes, credentials, and execution remain local by
   default. Cloud features are optional and require explicit user action.
4. **Keep humans authoritative.** Models request actions; the runtime validates, authorizes, and
   executes them. High-impact external actions require informed approval.
5. **Prove outcomes.** “Done” means the relevant build, lint, tests, runtime behavior, and diff have
   been checked and the evidence is visible.
6. **Make state inspectable.** Memory, routing decisions, permissions, usage, task status, and
   generated artifacts must be visible and, where applicable, editable, deletable, and exportable.
7. **Do not manufacture reality.** No fake metrics, quotas, terminals, agent progress, test results,
   provider responses, or production records. Use honest loading, empty, unavailable, and error
   states.
8. **Use one brand system.** Product surfaces follow [BRAND.md](BRAND.md), use the canonical Any
   Code assets, and carry the exact “Powered by heptagram-ai.com” attribution.

## Architecture rules

- Privileged operations cross the Tauri boundary and execute in the Rust runtime, never directly
  in the renderer.
- The UI never owns secrets, permissions, orchestration, or authoritative task state.
- Provider adapters translate normalized requests and responses; they do not plan tasks or execute
  tools.
- Every native tool, MCP server, connector, skill, and plugin enters through the capability
  registry and the same permission engine.
- Skills contain reusable instructions and declare requirements; they never grant permissions.
- Third-party executable plugins run out of process or in an equivalent sandbox with explicit
  filesystem, network, CPU, memory, and time limits.
- Untrusted repository text, terminal output, browser content, HTTP responses, MCP results, and
  model output are treated as data, not authority.
- Every privileged action and billable model request emits an auditable event.
- Long-running tasks are cancellable and resumable where technically possible.
- Indexing and retrieval are targeted; dumping an entire repository into a model is not an
  acceptable context strategy.

Full rationale and review gates are in [ARCHITECTURE.md](ARCHITECTURE.md) and
[SECURITY.md](SECURITY.md).

## Delivery rules

- Work only within the active phase in [ROADMAP.md](ROADMAP.md). Later-phase work is limited to a
  minimal interface or seam when omitting it would force a known architectural rewrite.
- Maintain existing staging VPS health checks and observability under
  [OPERATIONS.md](OPERATIONS.md). Maintenance does not authorize new Phase 9 product scope.
- Each change has one primary user outcome, explicit acceptance criteria, and a bounded diff.
- Foundations precede dependants: permissions before autonomous tools, provider contracts before
  provider UI, single-agent correctness before parallel agents, and capability sandboxing before a
  marketplace.
- Preserve existing architecture unless an accepted ADR changes it.
- Do not add empty crates, placeholder screens, speculative abstractions, or unused infrastructure.
- New dependencies require a concrete need, license compatibility, maintenance review, and a
  security justification proportionate to their privilege.
- Record incomplete work and limitations explicitly. Never label partial work complete.

## Security and privacy rules

- Secrets live in the operating-system credential store. They never appear in prompts, application
  state, analytics, logs, screenshots, fixtures, or committed configuration.
- Access is least-privilege, scoped to the current workspace and capability, and revocable.
- Destructive filesystem actions, force pushes, production deploys, production data mutations, and
  credential exposure are denied by default or require exact, contextual approval.
- Network use is visible and governed by user policy. Local Only mode must remain functional while
  offline after required local assets are installed.
- Telemetry is disclosed, controllable, minimized, and contains no source code or secrets by
  default.
- User-owned configuration, prompts, skills, memory, MCP configuration, workflows, and usage data
  are exportable. Deletion must be real and documented.
- Provider licenses, API terms, trademarks, and third-party code licenses must be respected.

## Quality and release rules

A feature is releasable only when the applicable items below are demonstrated:

- Functional acceptance criteria pass on real data or a real repository.
- Unit, integration, contract, end-to-end, and fault tests pass where the affected boundary calls
  for them.
- Loading, empty, error, cancellation, offline, and permission-denied states behave predictably.
- Dark, light, high-contrast, keyboard navigation, visible focus, and WCAG 2.2 AA requirements are
  checked for affected UI.
- No secret leakage, silent failure, unbounded process, blocking UI-thread I/O, or state corruption
  is observed.
- Documentation, migration notes, and the task's verification evidence are current.

The detailed checklist is [ENGINEERING-STANDARDS.md](ENGINEERING-STANDARDS.md). A skipped check is
a disclosed limitation, not a pass.

## Decision rules

- Use an ADR for decisions that are hard to reverse, cross more than one subsystem, change a trust
  boundary, introduce persistent data formats, or materially alter scope.
- Prefer the simplest design that satisfies the current phase without closing an approved future
  path.
- When requirements conflict, pause implementation, cite the conflicting sources, and update the
  governing document before proceeding.
- Product scope changes are intentional decisions, never side effects of implementation.
