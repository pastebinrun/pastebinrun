// SPDX-FileCopyrightText: 2022 - 2023 Konrad Borowski <konrad@borowski.pw>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

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
    editor,
  );
}
