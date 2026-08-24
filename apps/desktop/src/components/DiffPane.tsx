import { useQuery } from "@tanstack/react-query";
import { useEffect, useRef, useState } from "react";
import { commands } from "../lib/tauri";
import { languageForPath } from "../lib/language";
import { loadMonaco, type Monaco } from "../lib/monaco";
import { useWorkbenchStore } from "../state/workbenchStore";

export default function DiffPane({ path }: { path: string }) {
  const closeDiff = useWorkbenchStore((s) => s.closeDiff);
  const { data, isLoading, isError } = useQuery({
    queryKey: ["git-diff", path],
    queryFn: () => commands.gitDiff(path),
  });
  const containerRef = useRef<HTMLDivElement>(null);
  const diffEditorRef = useRef<ReturnType<Monaco["editor"]["createDiffEditor"]> | null>(null);
  const [monacoReady, setMonacoReady] = useState<Monaco | null>(null);

  useEffect(() => {
    let disposed = false;
    let dispose: (() => void) | null = null;

    loadMonaco().then((monaco) => {
      if (disposed || !containerRef.current) return;
      const editor = monaco.editor.createDiffEditor(containerRef.current, {
        automaticLayout: true,
        readOnly: true,
        minimap: { enabled: false },
        fontFamily: "JetBrains Mono, ui-monospace, monospace",
        fontSize: 13,
      });
      diffEditorRef.current = editor;
      setMonacoReady(monaco);
      dispose = () => editor.dispose();
    });

    return () => {
      disposed = true;
      dispose?.();
    };
  }, []);

  useEffect(() => {
    if (!data || !monacoReady || !diffEditorRef.current) return;
    const language = languageForPath(path);
    diffEditorRef.current.setModel({
      original: monacoReady.editor.createModel(data.headContent ?? "", language),
      modified: monacoReady.editor.createModel(data.workingContent ?? "", language),
    });
  }, [data, path, monacoReady]);

  const loading = isLoading || !monacoReady;

  return (
    <div style={{ display: "flex", flexDirection: "column", height: "100%" }}>
      <div
        style={{
          display: "flex",
          alignItems: "center",
          justifyContent: "space-between",
          padding: "6px 10px",
          borderBottom: "1px solid var(--ac-border)",
          fontSize: "0.8rem",
        }}
      >
        <span>
          {path} <span style={{ color: "var(--ac-text-muted)" }}>— HEAD vs working tree</span>
        </span>
        <button
          onClick={closeDiff}
          style={{ background: "none", border: "none", color: "var(--ac-text-muted)", cursor: "pointer" }}
        >
          Close ×
        </button>
      </div>
      {loading && !isError && <Centered text="Loading diff…" />}
      {isError && <Centered text="Could not load diff" />}
      <div style={{ flex: 1, minHeight: 0, display: loading || isError ? "none" : "block" }} ref={containerRef} />
    </div>
  );
}

function Centered({ text }: { text: string }) {
  return (
    <div style={{ flex: 1, display: "flex", alignItems: "center", justifyContent: "center", color: "var(--ac-text-muted)" }}>
      {text}
    </div>
  );
}
