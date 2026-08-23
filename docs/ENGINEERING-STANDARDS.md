# Engineering standards

## Definition of done

A feature is done when **all** of these hold (PRD §101). Partial completion is reported as partial.

- Functional implementation complete
- UI matches the design system; dark, light and high-contrast verified
- Keyboard navigation and visible focus verified (WCAG 2.2 AA)
- Loading, empty and error states implemented — all three
- Permissions enforced in the runtime, not the UI
- Unit tests pass; integration tests pass; the end-to-end path passes
- No secrets in logs, prompts, or telemetry
- Documentation updated

## Quality bar

Ship nothing containing: placeholder UI, fake metrics, invented provider quotas, simulated agent
state, a mock terminal, hardcoded demo records, unhandled loading or empty states, silent failures,
blocking filesystem work on the UI thread, or unbounded agent execution.

If real data is not available yet, the empty state *is* the feature.

## Testing

Every test or QA run is recorded in [`REVIEW.md`](../REVIEW.md) with its command or procedure,
environment, result, and limitations. Failed entries remain in the ledger after they are fixed.

| Layer | Required for |
|-------|--------------|
| Unit | Router, usage calculator, permission engine, context selection, provider adapters, event reducer, git operations, capability registry |
| Integration | Provider connections, MCP, terminal, git, database, cloud API, auth, sync |
| End-to-end (Playwright) | Onboarding, connect provider, open repo, send task, agent edits, diff, test run, approval, usage, restart-and-resume |
| Contract | Every provider adapter passes the same suite: auth, discovery, streaming, cancellation, tool calls, usage parsing, error normalisation, rate limits, timeout, fallback |
| Fault | Provider outage, offline, DB down, MCP/plugin/terminal/agent crash, sync interruption, rate limit, token expiry, disk full, repo deleted, git conflict |

Fault tests exist to prove one property: the application fails predictably rather than corrupting
state.

## Commits and branches

- `main` is protected and always releasable. Work on branches, merge via PR.
- Conventional commits: `feat:`, `fix:`, `docs:`, `refactor:`, `test:`, `chore:`.
- CI must be green before merge. Never merge with a skipped check.

## Performance targets

Warm launch ≤ 1.5s · navigation ≤ 100ms · keyboard and terminal input ≤ 16ms perceived ·
incremental index of one changed file ≤ 1s · idle memory < 300MB · 60 FPS · crash-free > 99.5%.

Large repository work runs off the UI thread. No exceptions.

## Release artifacts

- Native packages are built by GitHub Actions from a clean, tagged commit, not from a developer
  workstation.
- CI publishes only artifacts produced by successful test and packaging jobs.
- Release credentials remain in protected GitHub environments and are never available to pull
  requests or printed in logs.
- Every published package is signed where the platform supports signing, carries a SHA-256
  checksum, and records the source commit and application version.
- A platform is not advertised as supported until its package installs, launches, updates, and
  uninstalls successfully on a clean supported device or runner.
- Missing application targets produce no artifact; placeholder installers and empty packages are
  forbidden.

## Dependencies

Adding a dependency requires a reason that a few lines of code cannot satisfy. Prefer the standard
library, then an already-present dependency. Every new one is a supply-chain surface (PRD §90).
