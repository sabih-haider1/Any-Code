# QA and review ledger

This is the shared verification record for Any Code. Every agent that runs tests, performs QA, or
reviews a change appends a dated entry before reporting completion. Results are never rewritten
from fail to pass; a later rerun gets its own row so the history remains inspectable.

## Current quality status

| Area | Last result | Evidence |
|------|-------------|----------|
| Rust workspace | Pass | 26 tests passed on 2026-08-24 |
| Rust lint | Pass | Clippy completed with warnings denied on 2026-08-24 |
| TypeScript | Pass | Workspace typecheck passed on 2026-08-24 |
| Web production build | Pass with warning | Vite production build completed on 2026-08-24; large Monaco chunks remain |
| Rust formatting | Pass | `cargo fmt --all -- --check` on 2026-08-24 |
| Native desktop compile | Pass | Release build completed and executable launched on 2026-08-23 |
| macOS bundle | Pass | `.app` and unsigned `.dmg` produced locally on 2026-08-23 |
| Accessibility/static UI | Pass with limitations | Second-pass keyboard-source review completed on 2026-08-24; automated accessibility, screen-reader, and captured native-app walkthrough remain |
| Windows installer | Not run locally | GitHub Actions release job is the canonical Windows environment |
| GitHub CI | Pass | Run `32608603364` passed on Linux, macOS, and Windows |

## Review protocol

Each entry records:

- UTC timestamp and source commit
- Scope and environment
- Exact command or manual procedure
- Pass, fail, blocked, or not-run result
- Relevant counts and artifact paths
- Failures, warnings, and unverified limitations

A failed test remains visible. Fixing it requires a new passing entry with a link or reference to
the earlier failure.

## Verification history

### 2026-08-23T00:41:22Z — Brand integration baseline

- **Scope:** Any Code logo, Heptagram AI attribution, React shell, Tauri metadata, and repository
  documentation.
- **Source:** `e3801e7` plus the then-uncommitted branding/documentation change under review.
- **Environment:** macOS, Node 20.19.5, pnpm 9.15.9, local Rust toolchain.
- **Pass:** `pnpm lint` — TypeScript project references completed without errors.
- **Pass:** `pnpm build` — 31 modules transformed; production assets emitted to
  `apps/desktop/dist/`.
- **Pass:** `cargo test --workspace` — 5 passed, 0 failed.
- **Pass:** `cargo fmt --all --check` — no formatting changes required.
- **Pass:** `cargo clippy --workspace --all-targets --all-features -- -D warnings` — completed
  without warnings.
- **Pass:** `pnpm --filter @anycode/desktop exec tauri build --no-bundle` — optimized native
  executable produced at `apps/desktop/src-tauri/target/release/anycode-desktop`.
- **Pass:** native launch smoke test — the optimized executable started and exposed an on-screen
  `anycode-desktop` window.
- **Pass:** parallel `pnpm tauri build` packaging run — produced `Any Code.app` and
  `Any Code_0.1.0_x64.dmg`; process exited 0. The artifacts are unsigned and remain local build
  output.
- **Blocked:** real UI screenshot capture — macOS denied screen-capture/assistive access to the
  automation process. No mock screenshot was substituted.
- **Not run:** Windows installer execution, macOS notarization, screen-reader review, and full
  keyboard/accessibility walkthrough.

### 2026-08-23T00:45:09Z — GitHub CI after branding and governance push

- **Source:** `ab100dc2a8f49cd1755f0549d736ba5e6349209e` on `main`.
- **Pass:** [GitHub Actions run 32608603364](https://github.com/sabih-haider1/Any-Code/actions/runs/32608603364).
- **Pass:** committed-secret rejection.
- **Pass:** Rust formatting and Clippy with warnings denied.
- **Pass:** Rust workspace tests on Ubuntu, macOS, and Windows.
- **Pass:** frozen pnpm install and desktop frontend production build.
- **Warning:** GitHub reported that Node.js 20 action runtimes are deprecated and forced
  `actions/setup-node@v4` and `pnpm/action-setup@v4` onto Node 24. The run remained successful;
  action-version migration should be handled in a dedicated CI maintenance change.

### 2026-08-24 — Phase 1 workbench review and UX remediation

- **Scope:** Claude's Phase 1 explorer, editor, Git diff, terminal, command palette, settings,
  persistence, visual language, keyboard access, failure states, and workspace transitions.
- **Source:** `a831c23` plus the then-uncommitted workbench and model-provider changes under
  review. Unrelated concurrent changes were preserved.
- **Review — fail:** dirty editor tabs could be closed without confirmation; file-open,
  last-workspace, theme-save, terminal-write, and terminal-resize errors were silently swallowed;
  explorer and source-control rows were not keyboard controls; nested tab interactions used
  invalid button semantics; command palette and settings lacked complete dialog semantics; diff
  models were not disposed; opening another workspace retained stale diff state.
- **Review — fail:** the visual implementation was dominated by one-off inline styles, ambiguous
  text symbols, a purple AI-dashboard accent, no clear information hierarchy, and an app-wide
  fatal screen for recoverable runtime errors. The workbench did not consistently carry the Any
  Code identity or Heptagram AI attribution.
- **Remediated:** introduced a neutral, professional workbench hierarchy; restrained blue is used
  only for focus/selection/action; added a branded title bar and welcome flow; added explicit
  Explorer, Source Control, editor, terminal, status, settings, and command-palette regions; and
  retained the required “Powered by heptagram-ai.com” attribution on the welcome surface.
- **Remediated:** converted interactive rows to buttons, added landmarks and accessible names,
  dialog/listbox/tab semantics, visible focus, non-color status labels, Escape handling, empty and
  failure states, unsaved-change confirmation, save errors, safe workspace reset, and Monaco diff
  model cleanup.
- **Pass:** `pnpm lint` — TypeScript project references completed without errors.
- **Interrupted:** first `pnpm build` verification was manually stopped after Vite remained in its
  transform phase for more than two minutes. This result is retained rather than rewritten.
- **Pass:** repeated `pnpm build` — 1,313 modules transformed and production assets emitted in
  1m 54s.
- **Warning:** Vite reports oversized Monaco output, including a 7.03 MB TypeScript worker and a
  3.82 MB editor chunk. Lazy language loading/code splitting should be a focused performance task.
- **Pass:** `cargo fmt --all -- --check`.
- **Pass:** `cargo clippy --workspace --all-targets -- -D warnings`.
- **Pass:** `cargo test --workspace` — 26 passed, 0 failed, including core trust/event, filesystem
  boundary, Git, provider-stream parsing, persistence, and documentation targets.
- **Tooling gap:** `pnpm exec prettier --write ...` could not run because Prettier is not installed;
  no formatting claim is made for TypeScript/CSS beyond the successful compiler check.
- **Not run:** native visual capture, screen-reader walkthrough, signed package installation, and
  cross-platform installer execution. These require the relevant interactive/OS environments.

### 2026-08-24 — Second-pass UI/UX and ledger audit

- **Scope:** repeat source audit of runtime safety, focus management, keyboard operation,
  cross-platform labels, terminal lifecycle, product attribution, and ledger accuracy.
- **Review — fail:** `Explorer.tsx` contained a `useState` call at module scope. TypeScript accepted
  it, but React would throw an invalid-hook-call error while loading the module. The first-pass
  green TypeScript result therefore did not prove that the UI was runtime-safe.
- **Review — incomplete:** dialog roles had been added without focus containment or restoration.
  Terminal write/resize errors remained silent, xterm subscriptions were not explicitly disposed,
  and shortcut help showed only the macOS notation.
- **Review wording corrected:** the accessibility status now says “keyboard-source review,” not a
  completed interactive keyboard walkthrough. No automated accessibility or screen-reader result
  is claimed.
- **Remediated:** moved Explorer state into `FileRow`; added modal focus containment, Escape
  handling, and focus restoration; completed command-palette combobox semantics; surfaced terminal
  write/resize failures; disposed terminal subscriptions; made shortcut copy cross-platform; and
  added persistent Heptagram AI attribution to the workbench title bar.
- **Pass:** `pnpm lint` after remediation — TypeScript project references completed without errors.
- **Interrupted:** the second-pass `pnpm build` remained in Vite transformation for more than four
  minutes while another agent was concurrently running the same desktop build. It was stopped to
  avoid compounding resource contention. The earlier completed production build remains the latest
  build result; this run is not represented as a new pass or product failure.
- **Limitation:** the repository has no ESLint React Hooks rule, component test suite, automated
  accessibility runner, or native UI harness. The module-scope-hook defect demonstrates why the
  TypeScript-only `lint` script is insufficient and should be strengthened.

## Open QA risks

- Release artifacts are unsigned until protected Apple and Windows signing credentials are
  configured in GitHub.
- The Phase 1 workbench has no component or end-to-end UI suite yet.
- The `lint` script is TypeScript-only and does not enforce React Hooks or accessibility rules.
- Monaco is not yet split by language and makes the first production build slow and the package
  larger than necessary.
- VPS service health is owner-reported and has not been independently checked in this QA run.
