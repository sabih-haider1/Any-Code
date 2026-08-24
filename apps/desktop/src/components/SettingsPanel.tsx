import { applyTheme, type ThemeName } from "@anycode/design-tokens";
import { commands } from "../lib/tauri";
import { useWorkbenchStore } from "../state/workbenchStore";

const THEMES: ThemeName[] = ["system", "light", "dark", "high-contrast"];

export default function SettingsPanel({
  theme,
  onThemeChange,
  onClose,
}: {
  theme: ThemeName;
  onThemeChange: (theme: ThemeName) => void;
  onClose: () => void;
}) {
  const workspace = useWorkbenchStore((s) => s.workspace);

  const changeTheme = (next: ThemeName) => {
    applyTheme(next);
    onThemeChange(next);
    commands.setTheme(next).catch(() => {});
  };

  return (
    <div
      onClick={onClose}
      style={{ position: "fixed", inset: 0, background: "rgba(0,0,0,0.35)", zIndex: 100, display: "flex", justifyContent: "flex-end" }}
    >
      <div
        onClick={(e) => e.stopPropagation()}
        style={{
          width: 320,
          height: "100%",
          background: "var(--ac-surface)",
          borderLeft: "1px solid var(--ac-border)",
          padding: "var(--ac-space-5)",
        }}
      >
        <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center" }}>
          <h2 style={{ fontSize: "0.95rem", margin: 0 }}>Settings</h2>
          <button onClick={onClose} style={{ background: "none", border: "none", color: "var(--ac-text-muted)", cursor: "pointer" }}>
            Close ×
          </button>
        </div>

        <section style={{ marginTop: "var(--ac-space-5)" }}>
          <h3 style={{ fontSize: "0.8rem", color: "var(--ac-text-secondary)", fontWeight: 500 }}>Theme</h3>
          {THEMES.map((name) => (
            <label key={name} style={{ display: "block", marginBottom: "var(--ac-space-2)", fontSize: "0.85rem" }}>
              <input type="radio" name="theme" checked={theme === name} onChange={() => changeTheme(name)} /> {name}
            </label>
          ))}
        </section>

        <section style={{ marginTop: "var(--ac-space-6)" }}>
          <h3 style={{ fontSize: "0.8rem", color: "var(--ac-text-secondary)", fontWeight: 500 }}>Workspace</h3>
          <p style={{ fontSize: "0.8rem", color: "var(--ac-text-muted)", wordBreak: "break-all" }}>
            {workspace?.path ?? "No workspace open"}
          </p>
        </section>
      </div>
    </div>
  );
}
