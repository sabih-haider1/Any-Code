import { open as openFolderDialog } from "@tauri-apps/plugin-dialog";
import { applyTheme, type ThemeName } from "@anycode/design-tokens";
import { useCallback, useEffect, useState } from "react";
import CommandPalette, { type Command } from "./components/CommandPalette";
import DiffPane from "./components/DiffPane";
import EditorArea from "./components/EditorArea";
import Explorer from "./components/Explorer";
import GitPanel from "./components/GitPanel";
import SettingsPanel from "./components/SettingsPanel";
import StatusBar from "./components/StatusBar";
import TerminalPanel from "./components/TerminalPanel";
import WelcomeScreen from "./components/WelcomeScreen";
import { commands } from "./lib/tauri";
import { useWorkbenchStore } from "./state/workbenchStore";

const THEMES: ThemeName[] = ["system", "light", "dark", "high-contrast"];

export default function App() {
  const [theme, setTheme] = useState<ThemeName | null>(null);
  const [settingsOpen, setSettingsOpen] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const workspace = useWorkbenchStore((s) => s.workspace);
  const setWorkspace = useWorkbenchStore((s) => s.setWorkspace);
  const sidePanel = useWorkbenchStore((s) => s.sidePanel);
  const setSidePanel = useWorkbenchStore((s) => s.setSidePanel);
  const bottomPanelOpen = useWorkbenchStore((s) => s.bottomPanelOpen);
  const toggleBottomPanel = useWorkbenchStore((s) => s.toggleBottomPanel);
  const diffPath = useWorkbenchStore((s) => s.diffPath);
  const commandPaletteOpen = useWorkbenchStore((s) => s.commandPaletteOpen);
  const setCommandPaletteOpen = useWorkbenchStore((s) => s.setCommandPaletteOpen);

  useEffect(() => {
    commands
      .getTheme()
      .then((value) => {
        const loaded = (THEMES as string[]).includes(value) ? (value as ThemeName) : "system";
        setTheme(loaded);
        applyTheme(loaded);
      })
      .catch((err) => setError(String(err)));

    commands
      .getLastWorkspace()
      .then((info) => info && setWorkspace(info))
      .catch(() => {});
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const openFolder = useCallback(async () => {
    const selected = await openFolderDialog({ directory: true, multiple: false });
    if (typeof selected !== "string") return;
    try {
      const info = await commands.openWorkspace(selected);
      setWorkspace(info);
    } catch (err) {
      setError(String(err));
    }
  }, [setWorkspace]);

  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      const mod = e.metaKey || e.ctrlKey;
      if (mod && e.shiftKey && e.key.toLowerCase() === "p") {
        e.preventDefault();
        setCommandPaletteOpen(!commandPaletteOpen);
      }
    };
    window.addEventListener("keydown", handler);
    return () => window.removeEventListener("keydown", handler);
  }, [commandPaletteOpen, setCommandPaletteOpen]);

  if (error) {
    return (
      <main style={{ padding: "var(--ac-space-8)", color: "var(--ac-danger)" }}>
        Could not reach the local runtime: {error}
      </main>
    );
  }

  if (theme === null) {
    return <main style={{ padding: "var(--ac-space-8)" }}>Loading…</main>;
  }

  const paletteCommands: Command[] = [
    { id: "open-folder", label: "Workspace: Open Folder…", run: openFolder },
    { id: "toggle-terminal", label: "View: Toggle Terminal", run: toggleBottomPanel },
    { id: "show-explorer", label: "View: Show Explorer", run: () => setSidePanel("explorer") },
    { id: "show-git", label: "View: Show Source Control", run: () => setSidePanel("git") },
    { id: "open-settings", label: "Preferences: Open Settings", run: () => setSettingsOpen(true) },
  ];

  if (!workspace) {
    return (
      <div style={{ height: "100%" }}>
        <WelcomeScreen onOpenFolder={openFolder} />
        <CommandPalette commands={paletteCommands} />
      </div>
    );
  }

  return (
    <div style={{ display: "flex", flexDirection: "column", height: "100%" }}>
      <div style={{ display: "flex", flex: 1, minHeight: 0 }}>
        <nav
          style={{
            width: 40,
            borderRight: "1px solid var(--ac-border)",
            display: "flex",
            flexDirection: "column",
            alignItems: "center",
            paddingTop: "var(--ac-space-3)",
            gap: "var(--ac-space-2)",
            flexShrink: 0,
          }}
        >
          <SidebarIcon label="Explorer" active={sidePanel === "explorer"} onClick={() => setSidePanel("explorer")} symbol="⌘" />
          <SidebarIcon label="Source Control" active={sidePanel === "git"} onClick={() => setSidePanel("git")} symbol="⎇" />
        </nav>

        <aside style={{ width: 240, borderRight: "1px solid var(--ac-border)", flexShrink: 0 }}>
          {sidePanel === "explorer" ? <Explorer /> : <GitPanel />}
        </aside>

        <div style={{ flex: 1, minWidth: 0, display: "flex", flexDirection: "column" }}>
          <div style={{ flex: 1, minHeight: 0 }}>
            {diffPath ? <DiffPane path={diffPath} key={diffPath} /> : <EditorArea />}
          </div>
          {bottomPanelOpen && (
            <div style={{ height: 240, borderTop: "1px solid var(--ac-border)", flexShrink: 0 }}>
              <TerminalPanel />
            </div>
          )}
        </div>
      </div>

      <StatusBar onOpenSettings={() => setSettingsOpen(true)} />
      <CommandPalette commands={paletteCommands} />
      {settingsOpen && (
        <SettingsPanel theme={theme} onThemeChange={setTheme} onClose={() => setSettingsOpen(false)} />
      )}
    </div>
  );
}

function SidebarIcon({
  label,
  symbol,
  active,
  onClick,
}: {
  label: string;
  symbol: string;
  active: boolean;
  onClick: () => void;
}) {
  return (
    <button
      title={label}
      onClick={onClick}
      style={{
        width: 28,
        height: 28,
        border: "none",
        borderRadius: "var(--ac-radius-control-sm)",
        background: active ? "var(--ac-elevated)" : "transparent",
        color: active ? "var(--ac-text-primary)" : "var(--ac-text-muted)",
        cursor: "pointer",
        fontSize: "0.9rem",
      }}
    >
      {symbol}
    </button>
  );
}
