// SPDX-FileCopyrightText: 2022 - 2023 Konrad Borowski <konrad@borowski.pw>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import {
  Accessor,
  createResource,
  createSignal,
  JSXElement,
  Setter,
} from "solid-js";
import CodeView from "../models/CodeView";
import { getEditorTypeSignal } from "../options";
import TextAreaEditor from "./TextAreaEditor";

export default function Editor({
  code: initialCode,
  onInput,
  currentLanguage,
  form,
  setCodeView: setCodeView,
  setLabel,
}: {
  code: string;
  onInput(): void;
  currentLanguage: Accessor<string>;
  form: HTMLFormElement;
  setCodeView: Setter<CodeView>;
  setLabel: (e: JSXElement) => void;
}) {
  const [editorType] = getEditorTypeSignal();
  const [editorConstructor] = createResource(editorType, async (editorType) => {
    if (editorType === "textarea") {
      return null;
    } else {
      return (await import("./CodeMirrorEditor")).default;
    }
  });
  const [code, setCode] = createSignal<string>(initialCode);
  return (
    <>
      <input type="hidden" name="code" value={code()} />
      {editorConstructor()?.({
        code,
        setCode,
        onInput,
        currentLanguage,
        form,
        setCodeView,
        setLabel,
      }) || (
        <TextAreaEditor
          code={code()}
          setCode={setCode}
          setCodeView={setCodeView}
          onInput={onInput}
          setLabel={setLabel}
        />
      )}
    </>
  );
}
