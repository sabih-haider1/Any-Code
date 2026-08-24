import { useQuery } from "@tanstack/react-query";
import { commands } from "../lib/tauri";
import { useWorkbenchStore } from "../state/workbenchStore";
import { Icon } from "./Icons";

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
    <footer className="statusbar">
      <div className="status-group">
        <span>{workspace?.name ?? "No workspace"}</span>
        {branch && (
          <span className="status-button">
            <Icon name="branch" size={13} />
            {branch}
          </span>
        )}
      </div>
      <div className="status-group">
        <button
          onClick={toggleBottomPanel}
          className="status-button"
          aria-pressed={bottomPanelOpen}
        >
          <Icon name="terminal" size={13} />
          {bottomPanelOpen ? "Hide Terminal" : "Terminal"}
        </button>
        <button onClick={onOpenSettings} className="status-button">
          <Icon name="settings" size={13} />
          Settings
        </button>
      </div>
    </footer>
  );
}
