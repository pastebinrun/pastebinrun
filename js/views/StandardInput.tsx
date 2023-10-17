// SPDX-FileCopyrightText: 2022 - 2023 Konrad Borowski <konrad@borowski.pw>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

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
