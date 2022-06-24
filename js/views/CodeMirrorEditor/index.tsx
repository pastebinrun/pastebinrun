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

import { EditorView, basicSetup } from "codemirror";
import { indentWithTab } from "@codemirror/commands";
import { indentUnit } from "@codemirror/language";
import { Compartment } from "@codemirror/state";
import { keymap } from "@codemirror/view";
import {
  Accessor,
  createEffect,
  createUniqueId,
  JSXElement,
  onCleanup,
  Setter,
} from "solid-js";
import CodeView from "../../models/CodeView";
import { getTabIndentationSignal } from "../../options";
import "./codemirror.css";
import languagesMap from "./languages";

export default function CodeMirrorEditor({
  code,
  setCode,
  onInput,
  currentLanguage,
  form,
  setCodeView,
  setLabel,
}: {
  code: Accessor<string>;
  setCode: Setter<string>;
  onInput(): void;
  currentLanguage: Accessor<string>;
  form: HTMLFormElement;
  setCodeView: Setter<CodeView>;
  setLabel: Setter<JSXElement>;
}) {
  const [tabIndentationConfiguration] = getTabIndentationSignal();
  function getTabIndentationExtension() {
    return tabIndentationConfiguration() === "true"
      ? keymap.of([indentWithTab])
      : [];
  }
  const tabIndentation = new Compartment();
  const language = new Compartment();
  let avoidChangeNotifications = false;
  const labelId = createUniqueId();
  let view = new EditorView({
    doc: code(),
    extensions: [
      EditorView.contentAttributes.of({ "aria-labelledby": labelId }),
      tabIndentation.of(getTabIndentationExtension()),
      keymap.of([{ key: "Ctrl-Enter", run: () => true }]),
      basicSetup,
      EditorView.updateListener.of((v) => {
        if (v.docChanged && !avoidChangeNotifications) {
          onInput();
        }
      }),
      EditorView.lineWrapping,
      indentUnit.of(" ".repeat(4)),
      language.of([]),
    ],
  });
  createEffect(() => {
    view.dispatch({
      effects: tabIndentation.reconfigure(getTabIndentationExtension()),
    });
  });
  createEffect(async () => {
    const callback = languagesMap[currentLanguage()];
    const extension = callback ? await callback() : [];
    view.dispatch({
      effects: language.reconfigure(extension),
    });
  });
  const getValue = () => view.state.doc.toString();
  setCodeView({
    get code() {
      return getValue();
    },
    set code(code: string) {
      avoidChangeNotifications = true;
      view.dispatch({
        changes: { from: 0, to: view.state.doc.length, insert: code },
      });
      avoidChangeNotifications = false;
    },
  });

  setLabel(
    <label id={labelId} onClick={() => view.focus()}>
      {"Code: "}
    </label>
  );
  const submitCallback = () => setCode(getValue());
  form.addEventListener("submit", submitCallback);
  onCleanup(() => {
    form.removeEventListener("submit", submitCallback);
    submitCallback();
    view.destroy();
  });
  return view.dom;
}
