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
          {" Indent with tab"}
        </label>
      </p>
    </>
  );
}
