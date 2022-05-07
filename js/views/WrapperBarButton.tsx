// pastebin.run
// Copyright (C) 2022 Konrad Borowski
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

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
