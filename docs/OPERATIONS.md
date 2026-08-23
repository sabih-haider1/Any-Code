# Staging operations and monitoring

The project owner reports that online checks and application services are currently running on the
staging VPS. This is an operational environment, not evidence that the corresponding V1.5 cloud
product scope is complete. Its live health has not been independently verified in this repository.

## Operating boundary

- Local Only mode remains usable without the VPS.
- The VPS is the shared staging target for online checks, integration testing, and service
  monitoring.
- Existing staging operation is a cross-cutting maintenance responsibility even while Phase 0 is
  the active implementation phase.
- Building new cloud product features still follows the release boundary in
  [PRODUCT-SCOPE.md](PRODUCT-SCOPE.md).
- `.env.vps` is local deployment configuration. Never commit, print, quote, copy into prompts, or
  place its values in documentation.

## Expected staging services

The intended stack from PRD §73 is:

- Any Code API and reverse proxy
- PostgreSQL, Redis, and NATS
- Identity service
- Object storage when required
- Observability services

Documentation must distinguish **expected**, **deployed**, **healthy**, and **verified**. A service
is not marked healthy merely because it appears in configuration or a container list.

## Minimum online checks

Monitor and retain history for:

- Public endpoint reachability, TLS validity, and API health/readiness
- API latency, error rate, and request volume
- Database availability, connection pressure, query latency, storage, and backup status
- Redis and NATS availability, queue depth, delivery failures, and consumer lag
- Identity login/callback health without recording tokens or personal data
- Container restarts, CPU, memory, disk, and host availability
- Provider, model, agent, tool, MCP, plugin, sync, and cloud-worker failures when those components
  exist

Use OpenTelemetry for traces, Prometheus-compatible metrics, Grafana dashboards, and Loki logs as
specified in PRD §85. A hosted error tracker may supplement this stack but must follow the same
privacy and secret-redaction rules.

## Monitoring rules

1. Health checks are read-only and safe to repeat.
2. Dashboards and alerts use real telemetry; no synthetic success data or hardcoded green state.
3. Logs never contain provider keys, passwords, authorization headers, source code, prompts, or
   full user payloads by default.
4. Alerts identify the environment, service, symptom, start time, and runbook. They must not expose
   secrets in notification channels.
5. Availability checks run from outside the VPS as well as inside it, so reverse-proxy, DNS, and
   TLS failures are visible.
6. A deployment is incomplete until health checks and the relevant staging integration tests pass.
7. Monitoring access is least-privilege and auditable. Production and staging credentials are
   never shared with coding agents unless a specific, scoped operation requires them.
8. Any status recorded in Markdown includes the observation time and verification method; avoid
   permanent claims such as “all systems operational.”

## Incident response

When an online check fails:

1. Record the UTC time, affected service, user impact, and alert source.
2. Confirm the failure from a second signal before making destructive changes.
3. Preserve relevant redacted logs and traces.
4. Roll back the most recent deployment when it is the safest recovery path.
5. Verify service health and the affected user journey after recovery.
6. Document root cause and a prevention action for material incidents.

Do not restart, redeploy, migrate, or mutate VPS services solely to make a dashboard green. Those
actions require explicit operational authorization and verification appropriate to their risk.
