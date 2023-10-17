// SPDX-FileCopyrightText: 2022 - 2023 Konrad Borowski <konrad@borowski.pw>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import { onCleanup } from "solid-js";

export default function WrapperBarButton({
  index,
  runEvaluation,
  children,
}: {
  index: number;
  runEvaluation(): void;
  children: string;
}) {
  if (index === 0) {
    const globalKeyEvent = (e: KeyboardEvent) => {
      if (e.key === "Enter" && (e.ctrlKey || e.metaKey)) {
        e.preventDefault();
        runEvaluation();
      }
    };
    document.addEventListener("keydown", globalKeyEvent);
    onCleanup(() => document.removeEventListener("keydown", globalKeyEvent));
  }
  return (
    <button
      onClick={(e) => {
        e.preventDefault();
        runEvaluation();
      }}
    >
      {children}
    </button>
  );
}
