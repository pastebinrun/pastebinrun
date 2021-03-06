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
