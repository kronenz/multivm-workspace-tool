import { defineConfig } from "vite";

const host: string | undefined =
  typeof process !== "undefined"
    ? process.env.TAURI_DEV_HOST
    : undefined;

// https://vite.dev/config/
export default defineConfig(async () => ({
  // 1. prevent Vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // 3. tell Vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
}));
