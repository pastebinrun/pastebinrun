// SPDX-FileCopyrightText: 2022 - 2023 Konrad Borowski <konrad@borowski.pw>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

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
