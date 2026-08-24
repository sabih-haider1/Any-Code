export default function WelcomeScreen({ onOpenFolder }: { onOpenFolder: () => void }) {
  return (
    <div
      style={{
        height: "100%",
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        justifyContent: "center",
        gap: "var(--ac-space-4)",
      }}
    >
      <h1 style={{ fontSize: "1.2rem", fontWeight: 600 }}>Any Code</h1>
      <p style={{ color: "var(--ac-text-secondary)", margin: 0 }}>
        Any model. Any codebase. Any tool. One workspace.
      </p>
      <button
        onClick={onOpenFolder}
        style={{
          marginTop: "var(--ac-space-4)",
          padding: "var(--ac-space-2) var(--ac-space-5)",
          borderRadius: "var(--ac-radius-control)",
          border: "1px solid var(--ac-border)",
          background: "var(--ac-accent)",
          color: "white",
          fontSize: "0.9rem",
          cursor: "pointer",
        }}
      >
        Open Folder
      </button>
    </div>
  );
}
