import logo from "../../../../assets/brand/any-code-mark.png";
import { Icon } from "./Icons";
export default function WelcomeScreen({ onOpenFolder }: { onOpenFolder: () => void }) {
  return (
    <div className="welcome">
      <main className="welcome-main">
        <section className="welcome-card" aria-labelledby="welcome-title">
          <div className="welcome-brand">
            <img src={logo} alt="Any Code logo" />
            <div>
              <h1 id="welcome-title">Any Code</h1>
              <p>Local-first development workspace</p>
            </div>
          </div>
          <h2>Start with a repository</h2>
          <p>
            Open a project folder to browse files, edit code, review changes, and run commands in
            one focused workspace.
          </p>
          <div className="welcome-actions">
            <button className="button button--primary" onClick={onOpenFolder}>
              <Icon name="folder" />
              Open repository
            </button>
            <span className="shortcut">Ctrl/⌘⇧P</span>
          </div>
          <p className="welcome-hint">
            Your files remain on this computer. Any Code asks before connecting tools or services.
          </p>
        </section>
      </main>
      <footer className="powered-by">
        Powered by <a href="https://heptagram-ai.com">heptagram-ai.com</a>
      </footer>
    </div>
  );
}
