import { FitAddon } from "@xterm/addon-fit";
import { Terminal } from "@xterm/xterm";
import "@xterm/xterm/css/xterm.css";
import { listen } from "@tauri-apps/api/event";
import { useEffect, useRef } from "react";
import { commands } from "../lib/tauri";

function decodeBase64(data: string): Uint8Array {
  const binary = atob(data);
  const bytes = new Uint8Array(binary.length);
  for (let i = 0; i < binary.length; i++) bytes[i] = binary.charCodeAt(i);
  return bytes;
}

export default function TerminalPanel() {
  const containerRef = useRef<HTMLDivElement>(null);
  const sessionIdRef = useRef<string | null>(null);

  useEffect(() => {
    if (!containerRef.current) return;
    let disposed = false;
    let unlistenData: (() => void) | undefined;
    let unlistenExit: (() => void) | undefined;

    const term = new Terminal({
      fontFamily: "JetBrains Mono, ui-monospace, monospace",
      fontSize: 13,
      cursorBlink: true,
      theme: { background: "#00000000" },
    });
    const fit = new FitAddon();
    term.loadAddon(fit);
    term.open(containerRef.current);
    fit.fit();

    const resizeObserver = new ResizeObserver(() => fit.fit());
    resizeObserver.observe(containerRef.current);

    (async () => {
      const id = await commands.terminalSpawn(term.cols, term.rows);
      if (disposed) {
        await commands.terminalKill(id);
        return;
      }
      sessionIdRef.current = id;

      unlistenData = await listen<{ id: string; data: string }>(`terminal:data:${id}`, (event) => {
        term.write(decodeBase64(event.payload.data));
      });
      unlistenExit = await listen(`terminal:exit:${id}`, () => {
        term.write("\r\n[process exited]\r\n");
      });

      term.onData((data) => {
        commands.terminalWrite(id, data).catch(() => {});
      });
      term.onResize(({ cols, rows }) => {
        commands.terminalResize(id, cols, rows).catch(() => {});
      });
    })();

    return () => {
      disposed = true;
      resizeObserver.disconnect();
      unlistenData?.();
      unlistenExit?.();
      if (sessionIdRef.current) {
        commands.terminalKill(sessionIdRef.current).catch(() => {});
      }
      term.dispose();
    };
  }, []);

  return <div ref={containerRef} style={{ height: "100%", padding: "4px 8px" }} />;
}
