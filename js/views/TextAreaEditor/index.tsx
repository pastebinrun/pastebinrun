// SPDX-FileCopyrightText: 2022 - 2023 Konrad Borowski <konrad@borowski.pw>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import { createUniqueId, JSXElement, Setter } from "solid-js";
import CodeView from "../../models/CodeView";

export default function TextAreaEditor({
  onInput,
  code,
  setCode,
  setCodeView,
  setLabel,
}: {
  onInput: () => void;
  code: string;
  setCode: Setter<string>;
  setCodeView: Setter<CodeView>;
  setLabel: (e: JSXElement) => void;
}) {
  const id = createUniqueId();
  let textarea: HTMLTextAreaElement;
  setCodeView({
    get code() {
      return textarea.value;
    },
    set code(code: string) {
      textarea.value = code;
    },
  });
  setLabel(<label for={id}>{"Code: "}</label>);
  return (
    <textarea
      onInput={(e) => {
        onInput();
        setCode(e.currentTarget.value);
      }}
      ref={(e) => (textarea = e)}
      id={id}
    >
      {code}
    </textarea>
  );
}
