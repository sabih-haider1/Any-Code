# Security model

Any Code executes untrusted instructions against a developer's filesystem, shell, credentials and
production systems. Security is the architecture, not a feature.

## Threat model

| Threat | Control |
|--------|---------|
| Malicious repository instructions / prompt injection | Trust tagging. Repo text is `Trust::Untrusted` and cannot instruct |
| Malicious MCP server | Capability registry + per-capability permissions; MCP output is data |
| Malicious plugin | Sandboxed process or WASM, CPU/RAM/timeout/network/filesystem budgets, never in-process |
| Secret exfiltration | Secrets in OS keychain, injected into subprocesses, never into prompts or logs |
| Destructive shell execution | Risk classification + approval gates; the runtime decides, not the model |
| Compromised model response | Tool calls are validated against schemas and permissions before execution |
| OAuth token theft | Public-client PKCE, no embedded client secrets, envelope encryption when synced |
| Cross-workspace access | Filesystem permissions scoped to the workspace; writes outside require explicit grant |

## The one rule

**The model may request. The runtime decides.** No prompt, no repository file, no tool output and
no plugin can widen its own permissions. If a control can be talked out of by clever text, it is
not a control.

## Secrets

- Storage: macOS Keychain, Windows Credential Manager, Linux Secret Service.
- Never in application state, config files, logs, telemetry, analytics or prompts.
- Log redaction is automatic and tested — a test asserts a known secret does not appear in output.
- Agents receive scoped environment access. Never the whole `.env`, SSH keys, cloud root
  credentials, browser cookies or system keychains.
- `.env.vps` holds live staging credentials. It is gitignored, and CI fails the build if any
  `.env` file other than `.env.example` is committed.

## Permissions

States are `allow`, `ask`, `deny`. High-risk operations (production deploys, database drops, force
push, destructive removal) may be allowed once or per workspace — **never globally**.

Approval prompts state the exact command, the concrete risk, and the workspace. A prompt the user
cannot evaluate is not consent.

## Shell risk classes

| Class | Examples | Default |
|-------|----------|---------|
| Low | `ls`, `git status`, `npm test`, `cargo check` | allow |
| Medium | `npm install`, `git commit`, `docker compose up` | ask |
| High | `git push`, `terraform apply`, `kubectl delete`, migrations | ask, per-workspace only |
| Critical | broad `rm`, production database drop, secret extraction, privilege escalation | deny by default |

## Reporting

Security issues go to the repository owner privately. Do not open a public issue.
