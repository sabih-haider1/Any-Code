import { useEffect, useMemo, useState } from "react";
import { useWorkbenchStore } from "../state/workbenchStore";
import { useDialogFocus } from "../hooks/useDialogFocus";

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
  const dialogRef = useDialogFocus<HTMLDivElement>(open, () => setOpen(false));

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
    <div className="overlay palette-overlay" role="presentation" onClick={() => setOpen(false)}>
      <div
        className="palette"
        ref={dialogRef}
        role="dialog"
        aria-modal="true"
        aria-label="Command palette"
        onClick={(e) => e.stopPropagation()}
      >
        <input
          autoFocus
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          placeholder="Type a command…"
          role="combobox"
          aria-expanded="true"
          aria-autocomplete="list"
          onKeyDown={(e) => {
            if (e.key === "ArrowDown") {
              e.preventDefault();
              if (filtered.length) setHighlighted((h) => Math.min(h + 1, filtered.length - 1));
            } else if (e.key === "ArrowUp") {
              e.preventDefault();
              setHighlighted((h) => Math.max(h - 1, 0));
            } else if (e.key === "Enter") {
              run(filtered[highlighted]);
            } else if (e.key === "Escape") {
              setOpen(false);
            }
          }}
          aria-controls="command-results"
          aria-activedescendant={filtered[highlighted]?.id}
        />
        <div className="palette-list" id="command-results" role="listbox">
          {filtered.length === 0 && (
            <div
              style={{
                padding: "var(--ac-space-3) var(--ac-space-4)",
                color: "var(--ac-text-muted)",
              }}
            >
              No matching commands
            </div>
          )}
          {filtered.map((command, i) => (
            <button
              type="button"
              key={command.id}
              id={command.id}
              role="option"
              aria-selected={i === highlighted}
              className="palette-option"
              onMouseEnter={() => setHighlighted(i)}
              onClick={() => run(command)}
            >
              {command.label}
            </button>
          ))}
        </div>
      </div>
    </div>
  );
}
