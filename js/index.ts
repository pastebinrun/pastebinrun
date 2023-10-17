// SPDX-FileCopyrightText: 2022 - 2023 Konrad Borowski <konrad@borowski.pw>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import { render } from "solid-js/web";
import createEditor from "./create-editor";
import createOptionsLink from "./create-options-link";
import Options from "./views/Options";
import "../static/style-v2.css";

createOptionsLink();

const editor = document.getElementById("editor");
if (editor) {
  createEditor(editor);
}

const options = document.getElementById("options");
if (options) {
  options.textContent = "";
  render(Options, options);
}
