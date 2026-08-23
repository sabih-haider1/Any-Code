# Product scope

This document turns the vision in [PRD.md](../PRD.md) into a delivery boundary. It answers one
question: **what are we building now, next, and not yet?**

## Scope hierarchy

Use these levels when interpreting a request:

1. **North star** — the long-term direction in PRD §104–105. It guides architecture but does not
   authorize immediate implementation.
2. **Release scope** — V1, V1.5, or V2. It determines which capabilities belong to a release.
3. **Active phase** — the only phase whose product work may be implemented now. See
   [ROADMAP.md](ROADMAP.md).
4. **Accepted task** — a small, testable outcome inside the active phase.

A feature being described in the PRD does not mean it belongs in the active phase.

## Release boundaries

| Release | Included | Explicitly deferred |
|---------|----------|---------------------|
| **V1 · Local agentic desktop** | Phases 0–8: desktop foundation, workbench, provider abstraction, single-agent runtime, code intelligence, multi-agent isolation, browser verification, capability platform, and usage/budget intelligence | Cloud execution, cross-device continuation, mobile clients, enterprise controls |
| **V1.5 · Connected workspace** | Phase 9: identity, optional sync, remote sessions, web control center, cloud agents, team foundations, and first-party service integrations | Native mobile control plane and enterprise deployment |
| **V2 · Mobile and enterprise** | Phase 10 and PRD §9.3: iOS, Android, SSO/RBAC, organization policy, private marketplace, managed sandboxes and managed credits | General-purpose non-engineering automation unless separately approved |

PRD §9.1 mentions account, cloud sync, and staging foundations alongside V1 capabilities. For
delivery purposes, only the interfaces and seams needed to avoid a retrofit may be created during
V1. User-facing cloud behavior remains V1.5 scope. V1 must work completely in Local Only mode.
Existing VPS services and online checks may be maintained and monitored according to
[OPERATIONS.md](OPERATIONS.md); that operational duty does not move cloud features into V1.

## V1 product contract

V1 is a local-first Windows and macOS product. A new user can:

1. Install Any Code and open a real repository.
2. Connect at least one officially supported provider or local model.
3. Ask for a software change and inspect the context selected for it.
4. Review requested commands and grant or deny scoped permissions.
5. Observe edits and execution in a persistent task timeline.
6. Inspect the diff, test results, verification evidence, and actual model cost.
7. Restart the desktop app and resume the local session.

The contract is not met by a chat mock-up, an editor shell, or generated code without observed
verification.

## V1 non-goals

The following must not be pulled into V1 merely because they fit the long-term vision:

- Full VS Code extension compatibility or debugger parity
- A proprietary foundation model, Git host, CI service, or vector database product
- Mandatory Any Code cloud accounts or repository upload
- Native iOS or Android development environments
- General-purpose business agents outside software engineering
- Unsupported reuse or transfer of another vendor's consumer subscription
- Arbitrary unsandboxed plugin code in the main application process
- A marketplace before the capability and permission contracts are stable

## Scope test for every task

A task may enter implementation only when every answer is **yes**:

- Does it support the current phase's deliverables or exit condition?
- Is it necessary for the V1 product contract, security, or a documented architectural seam?
- Can it be completed and verified without pretending a later subsystem exists?
- Does it preserve provider independence, Local Only mode, and runtime-enforced permissions?
- Is there a concrete acceptance test and a clear stopping point?

If any answer is no, put the item in the appropriate future phase. Do not quietly expand the
current task.

## Change control

Scope changes require all of the following in the same change:

1. A written reason and user outcome.
2. The affected release and phase.
3. Security, privacy, cost, platform, and migration impact.
4. Updated acceptance criteria and roadmap.
5. An ADR when the decision is costly to reverse or crosses subsystem boundaries.

If documents disagree, stop and resolve the disagreement before implementation. Do not select the
interpretation that creates the most work.
