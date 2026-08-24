import { useQuery } from "@tanstack/react-query";
import { useState } from "react";
import { commands, type FsEntry } from "../lib/tauri";
import { useWorkbenchStore } from "../state/workbenchStore";

export default function Explorer() {
  return (
    <div className="panel">
      <div className="panel-header">Explorer</div>
      <div className="panel-scroll">
        <DirContents relative="" depth={0} />
      </div>
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
  if (isError) return <Row depth={depth} label="Could not read directory" danger />;
  if (data?.length === 0) return <Row depth={depth} label="This folder is empty" muted />;

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
      <Row
        depth={depth}
        label={entry.name}
        icon={open ? "▾" : "▸"}
        onClick={() => setOpen((v) => !v)}
      />
      {open && <DirContents relative={entry.path} depth={depth + 1} />}
    </>
  );
}

function FileRow({ entry, depth }: { entry: FsEntry; depth: number }) {
  const openTab = useWorkbenchStore((s) => s.openTab);
  const activePath = useWorkbenchStore((s) => s.activePath);
  const [error, setError] = useState(false);

  const open = async () => {
    try {
      const content = await commands.readFile(entry.path);
      openTab(entry.path, content);
    } catch {
      setError(true);
    }
  };

  return (
    <Row
      depth={depth}
      label={error ? `${entry.name} — cannot open as text` : entry.name}
      onClick={open}
      active={activePath === entry.path}
      danger={error}
    />
  );
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
  return onClick ? (
    <button
      type="button"
      className="row-button"
      aria-current={active || undefined}
      aria-expanded={icon ? icon === "▾" : undefined}
      onClick={onClick}
      style={{
        paddingLeft: `${depth * 14 + 8}px`,
        color: danger
          ? "var(--ac-danger)"
          : muted
            ? "var(--ac-text-muted)"
            : "var(--ac-text-primary)",
      }}
    >
      {icon && <span aria-hidden="true">{icon}</span>}
      <span className="row-label">{label}</span>
    </button>
  ) : (
    <div
      className={`row-button ${danger ? "danger" : "muted"}`}
      style={{ paddingLeft: `${depth * 14 + 8}px` }}
    >
      <span className="row-label">{label}</span>
    </div>
  );
}
