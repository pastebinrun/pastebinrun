// SPDX-FileCopyrightText: 2022 - 2023 Konrad Borowski <konrad@borowski.pw>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

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
