---
name: provider-adapter
description: Add or modify a model provider in Any Code — OpenAI, Anthropic, Gemini, OpenRouter, Ollama, Azure, Bedrock, Vertex, LM Studio, or any OpenAI-compatible endpoint. Use when implementing provider authentication, streaming, tool calls, model discovery, usage parsing, error normalisation, or fallback behaviour.
---

# Adding a model provider

The whole point of the abstraction: adding a provider must not touch the agent loop, the router, or
the UI. If it does, the abstraction leaked and that is the bug to fix first.

## Contract

Implement `ModelProvider` (PRD §79): `id()`, `models()`, `stream()`, `health()`, `capabilities()`.
Provider-specific request shapes, error codes, token field names and streaming framing stay inside
the adapter. What crosses the boundary is the normalised `ModelRequest`/`ModelStream` and a
normalised error.

## Required before merge

Every adapter passes the same contract suite. No provider gets an exemption.

- Authentication — each declared `authModes` path actually works
- Model discovery — returns real models from the account, never a hardcoded list
- Streaming — tokens arrive incrementally, not buffered and released at the end
- Cancellation — dropping the stream stops the upstream request
- Tool calls — round-trip through the normalised tool schema
- Usage parsing — input, output, cached and reasoning tokens mapped to `UsageEvent`
- Error normalisation — auth, rate limit, context overflow, server error, timeout each map to a
  distinct normalised variant the router can act on
- Rate limit handling — 429 with retry-after respected
- Timeout — bounded, configurable
- Fallback — the router can move the task to another provider without losing state

## Rules

- **Never invent numbers.** If the provider does not expose remaining quota, cost or a token count,
  report it as unknown. A plausible guess in a spend dashboard is a lie with a dollar sign on it.
- **Never reverse-engineer authentication.** No scraping cookies, browser sessions, private OAuth
  tokens, or another product's credential files. API keys, documented OAuth, enterprise credentials
  and local endpoints only. (PRD §18–20)
- **Never hardcode key formats.** Providers change them; validate by using the key, not by regex.
- **Emit a usage event per request**, at the adapter, including on failure.
- **Secrets come from the vault**, are never logged, and never appear in a prompt or an error
  message. Add a redaction test.

## Capabilities are declared, not assumed

`ProviderManifest` declares streaming, tools, vision, reasoning, usage API and cost API support.
The router reads these. An adapter that overstates its capabilities breaks routing for everyone.

## Local providers

Ollama, LM Studio and OpenAI-compatible endpoints are first-class, not a fallback tier. Privacy
First and Local Only routing depend on them working properly, offline, with zero cost accounting.
