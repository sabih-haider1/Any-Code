import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "@anycode/design-tokens/tokens.css";
import "./styles.css";

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      // Local filesystem/git reads — cheap to redo, wrong to treat as long-lived cache.
      staleTime: 2000,
      retry: false,
    },
  },
});

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <QueryClientProvider client={queryClient}>
      <App />
    </QueryClientProvider>
  </React.StrictMode>,
);
