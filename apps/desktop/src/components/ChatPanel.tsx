import { listen } from "@tauri-apps/api/event";
import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import {
  providerCommands,
  type ChatMessage,
  type ModelDefinition,
  type ProviderStatus,
} from "../lib/tauri";
import { Icon } from "./Icons";

/**
 * Proves Phase 2's exit condition end to end: pick a provider, pick a model, send a
 * message, watch it stream back — and switching providers changes nothing here except
 * which two dropdown values are selected. All provider-specific behavior stays inside
 * the Rust adapters; this component only ever sees the normalized chat/message shape.
 */
export default function ChatPanel() {
  const [providers, setProviders] = useState<ProviderStatus[] | null>(null);
  const [providersError, setProvidersError] = useState<string | null>(null);
  const [provider, setProvider] = useState<string | null>(null);
  const [models, setModels] = useState<ModelDefinition[] | null>(null);
  const [modelsError, setModelsError] = useState<string | null>(null);
  const [model, setModel] = useState<string | null>(null);
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [draft, setDraft] = useState("");
  const [pending, setPending] = useState<string | null>(null);
  const [sendError, setSendError] = useState<string | null>(null);
  const sessionId = useMemo(() => crypto.randomUUID(), []);
  const scrollRef = useRef<HTMLDivElement>(null);

  const refreshProviders = useCallback(() => {
    providerCommands
      .listProviders()
      .then((list) => {
        setProviders(list);
        setProvidersError(null);
        setProvider((current) => current ?? list.find((p) => p.hasKey)?.id ?? null);
      })
      .catch((error) => setProvidersError(String(error)));
  }, []);

  useEffect(refreshProviders, [refreshProviders]);

  useEffect(() => {
    if (!provider) {
      setModels(null);
      return;
    }
    setModel(null);
    setModelsError(null);
    providerCommands
      .listModels(provider)
      .then((list) => {
        setModels(list);
        setModel(list[0]?.id ?? null);
      })
      .catch((error) => setModelsError(String(error)));
  }, [provider]);

  useEffect(() => {
    scrollRef.current?.scrollTo({ top: scrollRef.current.scrollHeight });
  }, [messages]);

  const send = useCallback(async () => {
    if (!provider || !model || !draft.trim() || pending) return;
    const next = [...messages, { role: "user" as const, content: draft.trim() }];
    setMessages(next);
    setDraft("");
    setSendError(null);

    let requestId: string;
    try {
      requestId = await providerCommands.sendChat(provider, model, sessionId, next);
    } catch (error) {
      setSendError(String(error));
      return;
    }
    setPending(requestId);
    setMessages([...next, { role: "assistant", content: "" }]);

    const unlistenDelta = await listen<{ text: string }>(`chat:delta:${requestId}`, (event) => {
      setMessages((current) => {
        const updated = [...current];
        const last = updated[updated.length - 1];
        if (last?.role === "assistant") {
          updated[updated.length - 1] = { ...last, content: last.content + event.payload.text };
        }
        return updated;
      });
    });
    const unlistenDone = await listen(`chat:done:${requestId}`, () => {
      setPending(null);
      unlistenDelta();
      unlistenDone();
    });
    const unlistenError = await listen<{ message: string }>(`chat:error:${requestId}`, (event) => {
      setSendError(event.payload.message);
      setPending(null);
      unlistenDelta();
      unlistenDone();
      unlistenError();
    });
  }, [provider, model, draft, pending, messages, sessionId]);

  if (providersError) {
    return (
      <div className="empty-state">
        <div>
          <strong>Could not load providers</strong>
          {providersError}
        </div>
      </div>
    );
  }
  if (!providers) {
    return <div className="empty-state muted">Loading providers…</div>;
  }

  const connected = providers.filter((p) => p.hasKey);
  if (connected.length === 0) {
    return (
      <div className="empty-state">
        <div>
          <strong>No provider connected</strong>
          Add an API key in Settings → Providers, or install Ollama for a local model.
        </div>
      </div>
    );
  }

  return (
    <div className="panel">
      <div className="panel-subheader chat-toolbar">
        <label className="chat-select">
          Provider
          <select value={provider ?? ""} onChange={(e) => setProvider(e.target.value)}>
            {connected.map((p) => (
              <option key={p.id} value={p.id}>
                {p.name}
              </option>
            ))}
          </select>
        </label>
        <label className="chat-select">
          Model
          {modelsError ? (
            <span className="danger">{modelsError}</span>
          ) : (
            <select
              value={model ?? ""}
              onChange={(e) => setModel(e.target.value)}
              disabled={!models}
            >
              {models?.map((m) => (
                <option key={m.id} value={m.id}>
                  {m.displayName}
                </option>
              ))}
            </select>
          )}
        </label>
      </div>

      <div className="panel-scroll chat-messages" ref={scrollRef}>
        {messages.length === 0 && <p className="muted">Ask something to try {provider}.</p>}
        {messages.map((m, i) => (
          <div key={i} className={`chat-message chat-message--${m.role}`}>
            <span className="chat-role">{m.role}</span>
            <p>{m.content || (pending && i === messages.length - 1 ? "…" : "")}</p>
          </div>
        ))}
      </div>

      {sendError && (
        <p className="danger chat-error" role="alert">
          {sendError}
        </p>
      )}

      <form
        className="chat-input"
        onSubmit={(e) => {
          e.preventDefault();
          send();
        }}
      >
        <input
          value={draft}
          onChange={(e) => setDraft(e.target.value)}
          placeholder={model ? `Message ${model}…` : "Select a model first"}
          disabled={!model || !!pending}
          aria-label="Chat message"
        />
        <button
          type="submit"
          className="icon-button"
          disabled={!model || !draft.trim() || !!pending}
          aria-label="Send message"
        >
          <Icon name="send" />
        </button>
      </form>
    </div>
  );
}
