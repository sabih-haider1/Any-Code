---
name: definition-of-done
description: Verify an Any Code feature is actually finished before claiming completion, opening a PR, or merging. Use when a feature, screen, command or fix looks complete, when about to report work as done, or when reviewing someone else's change against the quality bar.
---

# Definition of done

A feature is not done because it renders. Run this before saying the word "done".

## Gate

- [ ] Functional implementation complete — no TODO in the happy path
- [ ] UI matches the design system; **dark, light and high-contrast** all verified visually
- [ ] Keyboard navigation works; focus is visible; screen-reader labels present (WCAG 2.2 AA)
- [ ] Loading state implemented
- [ ] Empty state implemented
- [ ] Error state implemented — including the failure that is most likely, not just the generic one
- [ ] Permissions enforced in the Rust runtime, not by hiding a button
- [ ] Unit tests pass
- [ ] Integration tests pass
- [ ] End-to-end path passes
- [ ] No secret appears in logs, prompts, telemetry or error messages
- [ ] Documentation updated

Report partial completion as partial. List what is not done and why. A checklist item skipped
silently is worse than one reported as skipped.

## Quality bar — automatic rejection

Ship none of these, ever:

placeholder UI · fake metrics · invented provider quotas · simulated agent state · mock terminal ·
mock production flows · hardcoded demo records · exposed secrets · unhandled loading states ·
unhandled empty states · silent failures · blocking filesystem work on the UI thread · unbounded
agent execution

When real data is not available yet, the **empty state is the feature**. Do not fill a dashboard
with plausible numbers to make a screenshot look finished.

## Evidence

Completion claims carry evidence: the test output, the command run, the screenshot, the observed
behaviour. "It should work" is not a verification result — run it and paste what happened,
including when it failed.
