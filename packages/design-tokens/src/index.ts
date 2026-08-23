export type ThemeName = "system" | "light" | "dark" | "high-contrast";

/**
 * Values mirrored from tokens.css for TypeScript consumers (e.g. computing contrast,
 * generating icon variants). CSS is the source of truth for anything the browser can
 * cascade itself — import "./tokens.css" for that instead of reading these at runtime.
 */
export const spacing = {
  1: "4px",
  2: "8px",
  3: "12px",
  4: "16px",
  5: "20px",
  6: "24px",
  8: "32px",
  10: "40px",
  12: "48px",
} as const;

export const radius = {
  controlSm: "6px",
  control: "8px",
  panel: "10px",
  modal: "12px",
} as const;

export const motion = {
  fast: "110ms",
  normal: "165ms",
  panel: "200ms",
  easing: "cubic-bezier(0.2, 0, 0, 1)",
} as const;

export const fonts = {
  ui: 'Inter, -apple-system, "Segoe UI", sans-serif',
  code: '"JetBrains Mono", ui-monospace, monospace',
} as const;

/** Applies the theme by setting the attribute tokens.css keys off — "system" clears it. */
export function applyTheme(theme: ThemeName, root: HTMLElement = document.documentElement): void {
  if (theme === "system") {
    root.removeAttribute("data-theme");
  } else {
    root.setAttribute("data-theme", theme);
  }
}
