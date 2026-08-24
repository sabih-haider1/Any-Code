import { useQuery } from "@tanstack/react-query";
import { useState } from "react";
import { commands, type FsEntry } from "../lib/tauri";
import { useWorkbenchStore } from "../state/workbenchStore";

export default function Explorer() {
  return (
    <div style={{ overflow: "auto", height: "100%", padding: "var(--ac-space-2)" }}>
      <DirContents relative="" depth={0} />
    </div>
  );
}

/** Renders the children of an already-open directory. Mounted only while expanded. */
function DirContents({ relative, depth }: { relative: string; depth: number }) {
  const { data, isLoading, isError } = useQuery({
    queryKey: ["dir", relative],
    queryFn: () => commands.listDir(relative),
  });

  if (isLoading) return <Row depth={depth} label="Loading…" muted />;
  if (isError) return <Row depth={depth} label="Could not read directory" muted danger />;

  return (
    <>
      {data?.map((entry) =>
        entry.isDir ? (
          <ExpandableDir key={entry.path} entry={entry} depth={depth} />
        ) : (
          <FileRow key={entry.path} entry={entry} depth={depth} />
        ),
      )}
    </>
  );
}

function ExpandableDir({ entry, depth }: { entry: FsEntry; depth: number }) {
  const [open, setOpen] = useState(false);
  return (
    <>
      <Row depth={depth} label={entry.name} icon={open ? "▾" : "▸"} onClick={() => setOpen((v) => !v)} />
      {open && <DirContents relative={entry.path} depth={depth + 1} />}
    </>
  );
}

function FileRow({ entry, depth }: { entry: FsEntry; depth: number }) {
  const openTab = useWorkbenchStore((s) => s.openTab);
  const activePath = useWorkbenchStore((s) => s.activePath);

  const open = async () => {
    try {
      const content = await commands.readFile(entry.path);
      openTab(entry.path, content);
    } catch {
      // Binary or unreadable file — nothing to show in a text editor. No tab opens.
    }
  };

  return <Row depth={depth} label={entry.name} onClick={open} active={activePath === entry.path} />;
}

function Row({
  depth,
  label,
  icon,
  onClick,
  active,
  muted,
  danger,
}: {
  depth: number;
  label: string;
  icon?: string;
  onClick?: () => void;
  active?: boolean;
  muted?: boolean;
  danger?: boolean;
}) {
  return (
    <div
      onClick={onClick}
      style={{
        paddingLeft: `${depth * 14 + 8}px`,
        paddingTop: "3px",
        paddingBottom: "3px",
        cursor: onClick ? "pointer" : "default",
        borderRadius: "var(--ac-radius-control-sm)",
        background: active ? "var(--ac-elevated)" : "transparent",
        color: danger ? "var(--ac-danger)" : muted ? "var(--ac-text-muted)" : "var(--ac-text-primary)",
        fontSize: "0.85rem",
        whiteSpace: "nowrap",
        overflow: "hidden",
        textOverflow: "ellipsis",
      }}
    >
      {icon && <span style={{ display: "inline-block", width: "1em" }}>{icon}</span>} {label}
    </div>
  );
}
