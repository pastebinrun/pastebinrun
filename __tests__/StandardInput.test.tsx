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

import { createSignal } from "solid-js";
import { render, screen } from "solid-testing-library";
import { expect, test } from "vitest";
import StandardInput from "../js/views/StandardInput";

test("StandardInput can become visible", async () => {
  const [visible, setVisible] = createSignal(false);
  const [, setStandardInput] = createSignal("");
  render(() => (
    <StandardInput visible={visible} setStandardInput={setStandardInput} />
  ));
  expect(screen.queryByRole("textbox")).toBeNull();
  setVisible(true);
  expect(screen.getByRole("textbox")).toBeEmptyDOMElement();
});
