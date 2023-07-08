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

import { createEffect, createSignal, onCleanup, Signal } from "solid-js";

export function createLocalStorageState(
  name: string,
  defaultValue: string,
): Signal<string> {
  const [state, setState] = createSignal(localStorage[name] || defaultValue);
  createEffect(() => {
    localStorage[name] = state();
  });
  function listener({ key, newValue }: StorageEvent) {
    if (key === name) {
      setState(newValue);
    }
  }
  addEventListener("storage", listener);
  onCleanup(() => removeEventListener("storage", listener));
  return [state, setState];
}

export function getEditorTypeSignal(): Signal<string> {
  return createLocalStorageState("editorType", "codemirror");
}

export function getTabIndentationSignal(): Signal<string> {
  return createLocalStorageState("tabIndentation", "false");
}
