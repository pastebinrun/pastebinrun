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

import { Setter } from "solid-js";
import CodeView from "../../models/CodeView";

export default function TextAreaEditor({
  onInput,
  code,
  setCode,
  setCodeView,
}: {
  onInput: () => void;
  code: string;
  setCode: Setter<string>;
  setCodeView: Setter<CodeView>;
}) {
  let textarea: HTMLTextAreaElement;
  setCodeView({
    get code() {
      return textarea.value;
    },
    set code(code: string) {
      textarea.value = code;
    },
  });
  return (
    <label>
      {"Code: "}
      <textarea
        onInput={(e) => {
          onInput();
          setCode(e.currentTarget.value);
        }}
        ref={textarea}
      >
        {code}
      </textarea>
    </label>
  );
}
