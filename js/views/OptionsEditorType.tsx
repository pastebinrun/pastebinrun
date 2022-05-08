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
