# Repository knowledge graph

Any Code uses [Graphify](https://graphify.com/docs) to give coding agents a shared, queryable map
of this repository. The graph supplements the source files; it does not replace them or override
the authority order in [PROJECT-RULES.md](PROJECT-RULES.md).

## Installed integration

- CLI package: `graphifyy` (command: `graphify`)
- Project skills: Codex, Claude Code, and generic `.agents/skills`
- Generated graph: [`graphify-out/`](../graphify-out/)
- Agent entry points: [`AGENTS.md`](../AGENTS.md) and [`CLAUDE.md`](../CLAUDE.md)

The graph is generated locally. The initial structural extraction does not require an account, API
key, or upload. Do not enable semantic extraction through a hosted model unless the repository
owner explicitly approves the provider, expected cost, and source-code disclosure boundary.

## Required agent workflow

Before answering a repository-wide question, an agent should query the graph:

```bash
graphify query "Where is permission enforcement defined?"
graphify explain "Event"
graphify path "Agent Runtime" "Capability Registry"
```

Use scoped graph results to choose which source files to inspect. The graph is derived data and may
be stale or incomplete, so architectural or security decisions must still be confirmed against the
governing Markdown and current source.

After changing code or project documentation, refresh the graph:

```bash
graphify update .
```

Then inspect `graphify-out/GRAPH_REPORT.md` and commit the updated graph artifacts with the change.
Graphify output changing after a real repository edit is expected.

## Installation for a new workstation

Python 3.10 or newer is required.

```bash
python3 -m pip install --user graphifyy
graphify install --project --platform codex
graphify install --project --platform claude
graphify install --project --platform agents
graphify hook install
graphify update .
```

The `graphify` executable must be on `PATH`. Do not place provider keys in repository files.

## What the graph may contain

Graph output can reproduce names and relationships from source, documentation, configuration, and
schemas. Treat it with the same confidentiality as the repository. Never publish `graphify-out/`
independently of the repository's access controls.
