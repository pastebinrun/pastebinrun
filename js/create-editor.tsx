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

import { render } from "solid-js/web";
import App from "./views/App";

export default function createEditor(editor: Element) {
  const markdown = editor.querySelector("#markdown") as Element;
  const languages = editor.querySelector("select") as HTMLSelectElement;
  const autoDelete = editor.querySelector(".autodelete-text") as Element;
  const rawPasteElement = editor.querySelector(".rawpaste-text") as Element;
  const code = (editor.querySelector("textarea") as HTMLTextAreaElement).value;
  editor.textContent = "";
  render(
    () => (
      <App
        markdown={markdown}
        languages={languages}
        autoDelete={autoDelete}
        rawPasteElement={rawPasteElement}
        code={code}
      />
    ),
    editor
  );
}
