import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// EXPERIMENTAL — NEVER MERGE. Static local bundle only; no dev server is
// used at rest and no remote origin is referenced.
export default defineConfig({
  root: "frontend",
  plugins: [react()],
  build: {
    outDir: "../dist",
    emptyOutDir: true,
    target: "es2020",
  },
  clearScreen: false,
});
