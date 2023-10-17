// SPDX-FileCopyrightText: 2022 - 2023 Konrad Borowski <konrad@borowski.pw>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

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
  setLabel: (e: JSXElement) => void;
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
  const view = new EditorView({
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
    </label>,
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
