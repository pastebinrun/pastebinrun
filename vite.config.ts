import { defineConfig } from "vite";
import solidPlugin from "vite-plugin-solid";

export default defineConfig({
  plugins: [solidPlugin()],
  build: {
    manifest: true,
    rollupOptions: {
      input: "js/index.ts",
    },
  },
});
