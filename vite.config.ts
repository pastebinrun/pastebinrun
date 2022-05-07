import { defineConfig } from "vite";
import solidPlugin from "vite-plugin-solid";

export default defineConfig({
  plugins: [solidPlugin()],
  build: {
    manifest: true,
    target: "esnext",
    polyfillDynamicImport: false,
    rollupOptions: {
      input: "js/index.ts",
    },
  },
});
