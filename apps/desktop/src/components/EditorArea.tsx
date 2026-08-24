import { useCallback, useEffect, useRef, useState } from "react";
import { commands } from "../lib/tauri";
import { languageForPath } from "../lib/language";
import { loadMonaco, type Monaco } from "../lib/monaco";
import { useWorkbenchStore } from "../state/workbenchStore";

export default function EditorArea() {
  const tabs = useWorkbenchStore((s) => s.tabs);
  const activePath = useWorkbenchStore((s) => s.activePath);
  const setActivePath = useWorkbenchStore((s) => s.setActivePath);
  const closeTab = useWorkbenchStore((s) => s.closeTab);

  if (tabs.length === 0) {
    return (
      <div
        style={{
          display: "flex",
          alignItems: "center",
          justifyContent: "center",
          height: "100%",
          color: "var(--ac-text-muted)",
        }}
      >
        Open a file from the explorer to start editing.
      </div>
    );
  }

  return (
    <div style={{ display: "flex", flexDirection: "column", height: "100%" }}>
      <div
        style={{
          display: "flex",
          borderBottom: "1px solid var(--ac-border)",
          overflowX: "auto",
          flexShrink: 0,
        }}
      >
        {tabs.map((tab) => (
          <button
            key={tab.path}
            onClick={() => setActivePath(tab.path)}
            style={{
              display: "flex",
              alignItems: "center",
              gap: "6px",
              padding: "6px 10px",
              border: "none",
              borderRight: "1px solid var(--ac-border)",
              background: activePath === tab.path ? "var(--ac-surface)" : "transparent",
              color: "var(--ac-text-primary)",
              fontSize: "0.8rem",
              cursor: "pointer",
              fontFamily: "var(--ac-font-ui)",
            }}
          >
            {tab.dirty && (
              <span style={{ width: 6, height: 6, borderRadius: "50%", background: "var(--ac-accent)" }} />
            )}
            {tab.path.split("/").pop()}
            <span
              onClick={(e) => {
                e.stopPropagation();
                closeTab(tab.path);
              }}
              style={{ color: "var(--ac-text-muted)", marginLeft: 4 }}
            >
              ×
            </span>
          </button>
        ))}
      </div>
      <div style={{ flex: 1, minHeight: 0 }}>
        {tabs.map((tab) => (
          <div key={tab.path} style={{ height: "100%", display: tab.path === activePath ? "block" : "none" }}>
            <MonacoPane path={tab.path} initialContent={tab.savedContent} />
          </div>
        ))}
      </div>
    </div>
  );
}

function MonacoPane({ path, initialContent }: { path: string; initialContent: string }) {
  const containerRef = useRef<HTMLDivElement>(null);
  const editorRef = useRef<ReturnType<Monaco["editor"]["create"]> | null>(null);
  const markDirty = useWorkbenchStore((s) => s.markDirty);
  const markSaved = useWorkbenchStore((s) => s.markSaved);
  const savedContentRef = useRef(initialContent);
  const [ready, setReady] = useState(false);

  useEffect(() => {
    let disposed = false;
    let disposeEditor: (() => void) | null = null;

    loadMonaco().then((monaco) => {
      if (disposed || !containerRef.current) return;
      const editor = monaco.editor.create(containerRef.current, {
        value: initialContent,
        language: languageForPath(path),
        automaticLayout: true,
        minimap: { enabled: false },
        fontFamily: "JetBrains Mono, ui-monospace, monospace",
        fontSize: 13,
      });
      editorRef.current = editor;
      setReady(true);

      const sub = editor.onDidChangeModelContent(() => {
        const dirty = editor.getValue() !== savedContentRef.current;
        markDirty(path, dirty);
      });

      disposeEditor = () => {
        sub.dispose();
        editor.dispose();
      };
    });

    return () => {
      disposed = true;
      disposeEditor?.();
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [path]);

  const save = useCallback(async () => {
    const editor = editorRef.current;
    if (!editor) return;
    const content = editor.getValue();
    await commands.writeFile(path, content);
    savedContentRef.current = content;
    markSaved(path, content);
  }, [path, markSaved]);

  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      if ((e.metaKey || e.ctrlKey) && e.key === "s") {
        e.preventDefault();
        save();
      }
    };
    window.addEventListener("keydown", handler);
    return () => window.removeEventListener("keydown", handler);
  }, [save]);

  return (
    <div style={{ height: "100%", position: "relative" }}>
      {!ready && (
        <div style={{ position: "absolute", inset: 0, display: "flex", alignItems: "center", justifyContent: "center", color: "var(--ac-text-muted)" }}>
          Loading editor…
        </div>
      )}
      <div ref={containerRef} style={{ height: "100%" }} />
    </div>
  );
}
