// SPDX-FileCopyrightText: 2022 - 2023 Konrad Borowski <konrad@borowski.pw>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import { getEditorTypeSignal, getTabIndentationSignal } from "../options";
import OptionsEditorType from "./OptionsEditorType";

export default function Options() {
  const editorTypeSignal = getEditorTypeSignal();
  const [tabIndentation, setTabIndentation] = getTabIndentationSignal();
  return (
    <>
      <p>
        {"Editor type: "}
        <OptionsEditorType
          name="Textarea"
          identifier="textarea"
          editorTypeSignal={editorTypeSignal}
        />{" "}
        <OptionsEditorType
          name="CodeMirror"
          identifier="codemirror"
          editorTypeSignal={editorTypeSignal}
        />
      </p>
      <p>
        <label title="Indenting with Ctrl+] is always possible, even if this option is disabled.">
          <input
            type="checkbox"
            onChange={(e) =>
              setTabIndentation(e.currentTarget.checked ? "true" : "false")
            }
            checked={JSON.parse(tabIndentation())}
          />
          {" Indent with tab when using CodeMirror"}
        </label>
      </p>
    </>
  );
}
