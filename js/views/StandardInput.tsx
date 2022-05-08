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

import { Accessor, Setter, Show } from "solid-js";

export default function StandardInput({
  visible,
  setStandardInput,
}: {
  visible: Accessor<boolean>;
  setStandardInput: Setter<string>;
}) {
  return (
    <Show when={visible()}>
      <details>
        <summary>Standard input</summary>
        <textarea
          name="stdin"
          onInput={(e) => setStandardInput(e.currentTarget.value)}
        />
      </details>
    </Show>
  );
}
