import { useEffect, useMemo, useState } from "react";
import { useWorkbenchStore } from "../state/workbenchStore";

export interface Command {
  id: string;
  label: string;
  run: () => void;
}

export default function CommandPalette({ commands }: { commands: Command[] }) {
  const open = useWorkbenchStore((s) => s.commandPaletteOpen);
  const setOpen = useWorkbenchStore((s) => s.setCommandPaletteOpen);
  const [query, setQuery] = useState("");
  const [highlighted, setHighlighted] = useState(0);

  const filtered = useMemo(
    () => commands.filter((c) => c.label.toLowerCase().includes(query.toLowerCase())),
    [commands, query],
  );

  useEffect(() => {
    if (!open) {
      setQuery("");
      setHighlighted(0);
    }
  }, [open]);

  useEffect(() => setHighlighted(0), [query]);

  if (!open) return null;

  const run = (command: Command | undefined) => {
    if (!command) return;
    command.run();
    setOpen(false);
  };

  return (
    <div
      onClick={() => setOpen(false)}
      style={{
        position: "fixed",
        inset: 0,
        background: "rgba(0,0,0,0.35)",
        display: "flex",
        justifyContent: "center",
        paddingTop: "12vh",
        zIndex: 100,
      }}
    >
      <div
        onClick={(e) => e.stopPropagation()}
        style={{
          width: 480,
          maxHeight: "50vh",
          background: "var(--ac-surface)",
          border: "1px solid var(--ac-border)",
          borderRadius: "var(--ac-radius-modal)",
          boxShadow: "0 12px 40px rgba(0,0,0,0.35)",
          display: "flex",
          flexDirection: "column",
          overflow: "hidden",
        }}
      >
        <input
          autoFocus
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          placeholder="Type a command…"
          onKeyDown={(e) => {
            if (e.key === "ArrowDown") {
              e.preventDefault();
              setHighlighted((h) => Math.min(h + 1, filtered.length - 1));
            } else if (e.key === "ArrowUp") {
              e.preventDefault();
              setHighlighted((h) => Math.max(h - 1, 0));
            } else if (e.key === "Enter") {
              run(filtered[highlighted]);
            } else if (e.key === "Escape") {
              setOpen(false);
            }
          }}
          style={{
            padding: "var(--ac-space-3) var(--ac-space-4)",
            border: "none",
            borderBottom: "1px solid var(--ac-border)",
            background: "transparent",
            color: "var(--ac-text-primary)",
            fontSize: "0.95rem",
            outline: "none",
          }}
        />
        <div style={{ overflow: "auto" }}>
          {filtered.length === 0 && (
            <div style={{ padding: "var(--ac-space-3) var(--ac-space-4)", color: "var(--ac-text-muted)" }}>
              No matching commands
            </div>
          )}
          {filtered.map((command, i) => (
            <div
              key={command.id}
              onMouseEnter={() => setHighlighted(i)}
              onClick={() => run(command)}
              style={{
                padding: "var(--ac-space-2) var(--ac-space-4)",
                cursor: "pointer",
                background: i === highlighted ? "var(--ac-elevated)" : "transparent",
                fontSize: "0.85rem",
              }}
            >
              {command.label}
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
