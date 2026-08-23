import { useCallback, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { applyTheme, type ThemeName } from "@anycode/design-tokens";

const THEMES: ThemeName[] = ["system", "light", "dark", "high-contrast"];

/**
 * Phase 0 shell. Proves the boundary end to end: React asks Rust for the persisted
 * theme, Rust reads it from the local SQLite store, and setting it round-trips back
 * through the same path — no privileged state lives in this component.
 */
export default function App() {
  const [theme, setThemeState] = useState<ThemeName | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    invoke<string>("get_theme")
      .then((value) => {
        const loaded = (THEMES as string[]).includes(value) ? (value as ThemeName) : "system";
        setThemeState(loaded);
        applyTheme(loaded);
      })
      .catch((err) => setError(String(err)));
  }, []);

  const changeTheme = useCallback((next: ThemeName) => {
    setThemeState(next);
    applyTheme(next);
    invoke("set_theme", { theme: next }).catch((err) => setError(String(err)));
  }, []);

  if (error) {
    return (
      <AppFrame>
        <ErrorState message={error} />
      </AppFrame>
    );
  }

  if (theme === null) {
    return (
      <AppFrame>
        <LoadingState />
      </AppFrame>
    );
  }

  return (
    <AppFrame>
      <section className="welcome" aria-labelledby="welcome-title">
        <p className="eyebrow">Universal agentic development environment</p>
        <h1 id="welcome-title">Any model. Any codebase. Any tool.</h1>
        <p className="welcome__summary">One secure, persistent engineering workspace.</p>
      </section>
      <fieldset
        className="theme-picker"
      >
        <legend>Theme</legend>
        {THEMES.map((name) => (
          <label key={name}>
            <input
              type="radio"
              name="theme"
              value={name}
              checked={theme === name}
              onChange={() => changeTheme(name)}
            />{" "}
            {name}
          </label>
        ))}
      </fieldset>
    </AppFrame>
  );
}

function LoadingState() {
  return <p role="status">Loading local workspace…</p>;
}

function ErrorState({ message }: { message: string }) {
  return (
    <p role="alert" style={{ color: "var(--ac-danger)" }}>
      Could not reach the local runtime: {message}
    </p>
  );
}

function AppFrame({ children }: { children: React.ReactNode }) {
  return (
    <div className="app-frame">
      <header className="brand-bar">
        <img className="brand-bar__mark" src="/any-code-mark.png" alt="" />
        <div>
          <div className="brand-bar__name">Any Code</div>
          <div className="brand-bar__signature" aria-label="Founder signature 207402">
            AC:32A2A
          </div>
        </div>
      </header>
      <main className="app-content">{children}</main>
      <footer className="powered-by">
        Powered by{" "}
        <a href="https://heptagram-ai.com" target="_blank" rel="noreferrer">
          heptagram-ai.com
        </a>
      </footer>
    </div>
  );
}
