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

import "./spinner.css";
import { OutputWrapper } from "./types";

const filterRegex =
  /(?:\t\.(?:text|file|section|globl|p2align|type|cfi_.*|size|section)\b|.Lfunc_end).*\n?/g;

export default class Output {
  split: HTMLDivElement;
  outputContainer: HTMLDivElement;
  output: HTMLDivElement;
  filterAsm = document.createElement("label");
  filterAsmCheckbox = document.createElement("input");
  wrapper: OutputWrapper | null = null;
  json: { output: string; status: number | null } | null = null;

  static addTo(split: HTMLDivElement) {
    const outputContainer = document.createElement("div");
    outputContainer.id = "outputcontainer";
    const output = document.createElement("div");
    output.id = "output";
    outputContainer.append(output);
    return new Output(split, outputContainer, output);
  }

  private constructor(
    split: HTMLDivElement,
    outputContainer: HTMLDivElement,
    output: HTMLDivElement
  ) {
    this.split = split;
    this.outputContainer = outputContainer;
    this.output = output;
    this.filterAsmCheckbox.type = "checkbox";
    this.filterAsmCheckbox.checked = true;
    this.filterAsmCheckbox.addEventListener("change", () => this.update());
    this.filterAsm.append(
      " ",
      this.filterAsmCheckbox,
      " Filter assembler directives"
    );
  }

  clear() {
    this.output.textContent = "";
    this.outputContainer.remove();
  }

  error() {
    this.output.textContent =
      "An error occured while running the code. Try again.";
  }

  display(
    wrapper: OutputWrapper,
    json: {
      output: string;
      status: number;
    }
  ) {
    this.wrapper = wrapper;
    this.json = json;
    this.update();
  }

  spin() {
    this.output.textContent = "";
    const spinner = document.createElement("div");
    spinner.className = "spinner";
    this.output.append(spinner);
    this.split.append(this.outputContainer);
  }

  update() {
    let { output, status } = this.json;
    this.clear();
    this.split.append(this.outputContainer);
    const outputHeader = document.createElement("div");
    outputHeader.className = "stdout-header";
    if (status) {
      const outputHeaderH2 = document.createElement("h2");
      outputHeaderH2.textContent += `Output (exit code ${status})`;
      outputHeader.append(outputHeaderH2);
    }
    const { isAsm, isFormatter } = this.wrapper;
    if (isAsm) {
      outputHeader.append(this.filterAsm);
    }
    const stdoutElement = document.createElement("pre");
    let anyError = false;
    if (output) {
      if (isAsm && this.filterAsmCheckbox.checked) {
        output = output.replace(filterRegex, "");
      }
      const rootElement = isFormatter
        ? document.createElement("span")
        : stdoutElement;
      let currentElem: HTMLElement = rootElement;
      const iter = output[Symbol.iterator]();
      while (true) {
        const next = iter.next();
        if (next.done) {
          break;
        }
        if (next.value === "\x7F") {
          switch (iter.next().value) {
            case "E":
              anyError = true;
              const error = document.createElement("span");
              error.className = "error";
              stdoutElement.append(error);
              currentElem = error;
              break;
            case "O":
              currentElem = rootElement;
              break;
            case "\x7F":
              currentElem.append("\x7F");
              break;
          }
        } else {
          currentElem.append(next.value);
        }
      }
    } else {
      const italic = document.createElement("i");
      italic.textContent = "(no output)";
      stdoutElement.append(italic);
    }
    if (anyError || !isFormatter) {
      this.output.append(outputHeader, stdoutElement);
    }
  }
}
