// SPDX-FileCopyrightText: 2022 - 2023 Konrad Borowski <konrad@borowski.pw>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import { createSignal } from "solid-js";
import { render, screen } from "@solidjs/testing-library";
import "@testing-library/jest-dom/vitest";
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
