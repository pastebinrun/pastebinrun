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
