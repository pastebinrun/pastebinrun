// SPDX-FileCopyrightText: 2022 - 2023 Konrad Borowski <konrad@borowski.pw>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

export default function createOptionsLink() {
  const li = document.createElement("li");
  const a = document.createElement("a");
  a.textContent = "Options";
  a.href = "/config";
  li.append(a);
  (document.querySelector("#menu-buttons ul") as Element).append(li);
}
