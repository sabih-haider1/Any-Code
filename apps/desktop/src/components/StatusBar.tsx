import { useQuery } from "@tanstack/react-query";
import { commands } from "../lib/tauri";
import { useWorkbenchStore } from "../state/workbenchStore";

export default function StatusBar({ onOpenSettings }: { onOpenSettings: () => void }) {
  const workspace = useWorkbenchStore((s) => s.workspace);
  const bottomPanelOpen = useWorkbenchStore((s) => s.bottomPanelOpen);
  const toggleBottomPanel = useWorkbenchStore((s) => s.toggleBottomPanel);
  const { data: branch } = useQuery({
    queryKey: ["git-branch"],
    queryFn: commands.gitBranch,
    enabled: !!workspace,
  });

  return (
    <div
      style={{
        display: "flex",
        alignItems: "center",
        justifyContent: "space-between",
        height: 26,
        padding: "0 var(--ac-space-3)",
        borderTop: "1px solid var(--ac-border)",
        background: "var(--ac-elevated)",
        fontSize: "0.75rem",
        color: "var(--ac-text-secondary)",
        flexShrink: 0,
      }}
    >
      <div style={{ display: "flex", gap: "var(--ac-space-4)" }}>
        <span>{workspace?.name ?? "No workspace"}</span>
        {branch && <span>⎇ {branch}</span>}
      </div>
      <div style={{ display: "flex", gap: "var(--ac-space-4)" }}>
        <button
          onClick={toggleBottomPanel}
          style={{ background: "none", border: "none", color: "inherit", cursor: "pointer", font: "inherit" }}
        >
          {bottomPanelOpen ? "Hide Terminal" : "Show Terminal"}
        </button>
        <button
          onClick={onOpenSettings}
          style={{ background: "none", border: "none", color: "inherit", cursor: "pointer", font: "inherit" }}
        >
          Settings
        </button>
      </div>
    </div>
  );
}
