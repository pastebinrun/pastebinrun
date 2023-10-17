// SPDX-FileCopyrightText: 2022 - 2023 Konrad Borowski <konrad@borowski.pw>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import { Signal } from "solid-js";

export default function OptionsEditorType({
  name,
  identifier,
  editorTypeSignal: [editor, setEditor],
}: {
  name: string;
  identifier: string;
  editorTypeSignal: Signal<string>;
}) {
  return (
    <label>
      <input
        type="radio"
        name="current-editor"
        checked={identifier === editor()}
        onChange={() => setEditor(identifier)}
      />
      {` ${name}`}
    </label>
  );
}
