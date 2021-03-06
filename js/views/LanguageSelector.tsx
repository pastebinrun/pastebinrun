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

import { For, Setter } from "solid-js";

export default function LanguageSelector({
  setCurrentLanguage,
  languages,
}: {
  setCurrentLanguage: Setter<string>;
  languages: HTMLSelectElement;
}) {
  return (
    <div class="group">
      <label>
        {"Language: "}
        <select
          id="language"
          name="language"
          onChange={(e) =>
            setCurrentLanguage(e.currentTarget.selectedOptions[0].value)
          }
        >
          <For each={[...languages.options]}>
            {(option) => (
              <option value={option.value} selected={option.selected}>
                {option.textContent}
              </option>
            )}
          </For>
        </select>
      </label>
    </div>
  );
}
