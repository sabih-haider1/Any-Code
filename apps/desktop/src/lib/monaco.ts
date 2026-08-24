import type * as MonacoModule from "monaco-editor";

export type Monaco = typeof MonacoModule;

let loadPromise: Promise<Monaco> | null = null;

/**
 * Monaco is ~4MB minified — loading it eagerly would blow the app's warm-launch budget
 * (docs/ENGINEERING-STANDARDS.md: ≤1.5s). It's dynamically imported here on first use
 * (the first file or diff a user actually opens) instead of at app startup, and workers
 * are self-hosted rather than fetched from a CDN — required for offline/Local Only mode
 * (docs/ARCHITECTURE.md invariant #7).
 */
export function loadMonaco(): Promise<Monaco> {
  if (!loadPromise) {
    loadPromise = (async () => {
      const [monaco, editorWorker, jsonWorker, cssWorker, htmlWorker, tsWorker] = await Promise.all(
        [
          import("monaco-editor"),
          import("monaco-editor/esm/vs/editor/editor.worker?worker"),
          import("monaco-editor/esm/vs/language/json/json.worker?worker"),
          import("monaco-editor/esm/vs/language/css/css.worker?worker"),
          import("monaco-editor/esm/vs/language/html/html.worker?worker"),
          import("monaco-editor/esm/vs/language/typescript/ts.worker?worker"),
        ],
      );

      self.MonacoEnvironment = {
        getWorker(_workerId: string, label: string) {
          switch (label) {
            case "json":
              return new jsonWorker.default();
            case "css":
            case "scss":
            case "less":
              return new cssWorker.default();
            case "html":
            case "handlebars":
            case "razor":
              return new htmlWorker.default();
            case "typescript":
            case "javascript":
              return new tsWorker.default();
            default:
              return new editorWorker.default();
          }
        },
      };

      return monaco;
    })();
  }
  return loadPromise;
}
