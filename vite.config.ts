// SPDX-FileCopyrightText: 2022 - 2023 Konrad Borowski <konrad@borowski.pw>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

/// <reference types="vitest" />
/// <reference types="vite/client" />

import { defineConfig } from "vite";
import solidPlugin from "vite-plugin-solid";

export default defineConfig({
  test: {
    environment: "jsdom",
  },
  plugins: [solidPlugin()],
  build: {
    manifest: true,
    rollupOptions: {
      input: "js/index.ts",
    },
  },
});
