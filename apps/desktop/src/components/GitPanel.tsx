import { useQuery } from "@tanstack/react-query";
import { commands, type GitFileStatus } from "../lib/tauri";
import { useWorkbenchStore } from "../state/workbenchStore";

const STATUS_LABEL: Record<GitFileStatus, string> = {
  modified: "M",
  added: "A",
  deleted: "D",
  renamed: "R",
  untracked: "U",
  conflicted: "!",
};

const STATUS_COLOR: Record<GitFileStatus, string> = {
  modified: "var(--ac-warning)",
  added: "var(--ac-success)",
  deleted: "var(--ac-danger)",
  renamed: "var(--ac-info)",
  untracked: "var(--ac-text-muted)",
  conflicted: "var(--ac-danger)",
};

export default function GitPanel() {
  const { data: branch } = useQuery({ queryKey: ["git-branch"], queryFn: commands.gitBranch });
  const { data: status, isLoading, isError } = useQuery({
    queryKey: ["git-status"],
    queryFn: commands.gitStatus,
    refetchInterval: 5000,
  });
  const openDiff = useWorkbenchStore((s) => s.openDiff);

  return (
    <div style={{ height: "100%", display: "flex", flexDirection: "column" }}>
      <div
        style={{
          padding: "var(--ac-space-2) var(--ac-space-3)",
          fontSize: "0.8rem",
          color: "var(--ac-text-secondary)",
          borderBottom: "1px solid var(--ac-border)",
        }}
      >
        {branch ?? "no branch"}
      </div>
      <div style={{ overflow: "auto", flex: 1 }}>
        {isLoading && <Empty text="Loading…" />}
        {isError && <Empty text="Not a git repository" />}
        {status?.length === 0 && <Empty text="No changes" />}
        {status?.map((entry) => (
          <div
            key={entry.path}
            onClick={() => openDiff(entry.path)}
            style={{
              display: "flex",
              alignItems: "center",
              gap: "var(--ac-space-2)",
              padding: "3px var(--ac-space-3)",
              cursor: "pointer",
              fontSize: "0.85rem",
            }}
          >
            <span style={{ color: STATUS_COLOR[entry.status], width: "1em", fontWeight: 600 }}>
              {STATUS_LABEL[entry.status]}
            </span>
            <span style={{ overflow: "hidden", textOverflow: "ellipsis", whiteSpace: "nowrap" }}>
              {entry.path}
            </span>
          </div>
        ))}
      </div>
    </div>
  );
}

function Empty({ text }: { text: string }) {
  return (
    <div style={{ padding: "var(--ac-space-3)", color: "var(--ac-text-muted)", fontSize: "0.85rem" }}>
      {text}
    </div>
  );
}
