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

import { getCurrentEditor, setCurrentEditor, types } from "../../editor-types";
import "./config-page.css";

export default async function createSettings(node: HTMLElement) {
  node.textContent = "Loading settings\u2026";
  const currentEditor = getCurrentEditor();
  node.textContent = "Editor type: ";
  for (const id in types) {
    const label = document.createElement("label");
    const radio = document.createElement("input");
    radio.type = "radio";
    radio.name = "current-editor";
    radio.checked = id === currentEditor;
    radio.addEventListener("change", async () => {
      await setCurrentEditor(id);
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
    });
    label.append(radio, ` ${types[id].name}`);
    node.append(label);
  }
}
