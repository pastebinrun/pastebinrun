// pastebin.run
// Copyright (C) 2020 Konrad Borowski
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
  getCurrentEditor,
  getTabIndentationConfiguration,
  setCurrentEditor,
  types,
} from "../../editor-types";
import "./config-page.css";

export default async function createSettings(node: HTMLElement) {
  function showConfigurationSuccess() {
    const success = document.createElement("p");
    success.className = "success";
    success.textContent = "Configuration was updated";
    node.append(success);
    setTimeout(() => {
      success.style.opacity = "0";
      setTimeout(() => {
        success.remove();
      }, 1 * 1000);
    }, 3 * 1000);
  }
  function showTabIndentation(editor: string) {
    tabIndentation.style.display = editor === "codemirror" ? "" : "none";
  }
  node.textContent = "";
  let currentEditor = getCurrentEditor();
  const editorType = document.createElement("p");
  editorType.textContent = "Editor type: ";
  for (const id in types) {
    const label = document.createElement("label");
    const radio = document.createElement("input");
    radio.type = "radio";
    radio.name = "current-editor";
    radio.checked = id === currentEditor;
    radio.addEventListener("change", () => {
      setCurrentEditor(id);
      showTabIndentation(id);
      showConfigurationSuccess();
    });
    label.append(radio, ` ${types[id].name}`);
    editorType.append(label);
  }
  const tabIndentation = document.createElement("p");
  const tabIndentationLabel = document.createElement("label");
  tabIndentationLabel.title =
    "Indenting with Ctrl+] is always possible, even if this option is disabled.";
  const tabIndentationInput = document.createElement("input");
  tabIndentationInput.type = "checkbox";
  tabIndentationInput.checked = getTabIndentationConfiguration();
  tabIndentationInput.addEventListener("change", function () {
    localStorage.setItem("tabIndentation", this.checked ? "true" : "false");
    showConfigurationSuccess();
  });
  tabIndentationLabel.append(tabIndentationInput, " Indent with tab");
  tabIndentation.append(tabIndentationLabel);
  showTabIndentation(currentEditor);
  node.append(editorType, tabIndentation);
}
