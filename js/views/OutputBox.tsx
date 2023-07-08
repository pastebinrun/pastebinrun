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

import { For, Show } from "solid-js";
import Wrapper from "../models/Wrapper";
import { createLocalStorageState } from "../options";

function parseOutput(wrapper: Wrapper, output: string) {
  return (
    <For
      each={("O" + output)
        .split("\x7F")
        .filter(
          (x) => x.length > 1 && (!wrapper.isFormatter || !x.startsWith("O")),
        )}
      fallback={<i>(no output)</i>}
    >
      {(item) => {
        const firstChar = item[0];
        if (firstChar === "O") {
          return item.substring(1);
        }
        if (firstChar === "E") {
          return <span class="error">{item.substring(1)}</span>;
        }
        return item;
      }}
    </For>
  );
}

function runAsmFilter(output: string) {
  return output.replace(
    /(?:\t\.(?:text|file|section|globl|p2align|type|cfi_.*|size|section)\b|.Lfunc_end).*\n?/g,
    "",
  );
}

export default function OutputBox({
  output,
  wrapper,
}: {
  output: { output: string; status: number | null };
  wrapper: Wrapper;
}) {
  const [filterAsmDirectivesOrig, setFilterAsmDirectives] =
    createLocalStorageState("filterAsmDirectives", "true");
  const filterAsmDirectives = () => JSON.parse(filterAsmDirectivesOrig());
  return (
    <>
      <div class="stdout-header">
        <Show when={output.status}>
          <h2>{`Output (exit code ${output.status})`}</h2>
        </Show>
        <Show when={wrapper.isAsm}>
          <label>
            <input
              type="checkbox"
              checked={filterAsmDirectives()}
              onChange={(e) =>
                setFilterAsmDirectives(e.currentTarget.checked.toString())
              }
            />
            {" Filter assembler directives"}
          </label>
        </Show>
      </div>
      <pre>
        {parseOutput(
          wrapper,
          wrapper.isAsm && filterAsmDirectives()
            ? runAsmFilter(output.output)
            : output.output,
        )}
      </pre>
    </>
  );
}
