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

import { createSignal, Show } from "solid-js";
import WrapperOptions from "../models/WrapperOptions";
import Editor from "./Editor";
import LanguageSelector from "./LanguageSelector";
import Output from "./Output";
import StandardInput from "./StandardInput";
import WrapperBar from "./WrapperBar";

export default function App({
  markdown,
  languages,
  autoDelete,
  rawPasteElement,
  code,
}: {
  markdown: Element | null;
  languages: HTMLSelectElement;
  autoDelete: Element | null;
  rawPasteElement: Element | null;
  code: string;
}) {
  let form: HTMLFormElement | undefined;
  const [isPaste, setIsPaste] = createSignal(true);
  const [currentLanguage, setCurrentLanguage] = createSignal(
    languages.selectedOptions[0]?.value
  );
  const [standardInputVisible, setStandardInputVisible] = createSignal(false);
  const [codeView, setCodeView] = createSignal({ code: "" });
  const [standardInput, setStandardInput] = createSignal("");
  const [wrapperOptions, setWrapperOptions] = createSignal<WrapperOptions>();
  const [label, setLabel] = createSignal<Element>();
  return (
    <form action="/" method="post" ref={(e) => (form = e)}>
      {markdown}
      <Show when={isPaste()}>{autoDelete}</Show>
      <div id="toolbar">
        <LanguageSelector
          setCurrentLanguage={setCurrentLanguage}
          languages={languages}
        />
        <WrapperBar
          currentLanguage={currentLanguage}
          setStandardInputVisible={setStandardInputVisible}
          codeView={codeView}
          runEvaluation={(wrapper, compilerOptions) => {
            setWrapperOptions();
            setWrapperOptions({ wrapper, compilerOptions });
          }}
        />
        <Show when={isPaste()}>{rawPasteElement}</Show>
        <span id="right-buttons">
          <button type="submit" name="share" value="share24">
            Share (delete after 24 hours)
          </button>{" "}
          <button type="submit" name="share" value="share">
            Share
          </button>
        </span>
      </div>
      <div id="split">
        <div id="extrafieldsplit">
          <div>{label}</div>
          <div id="textarea">
            <Editor
              code={code}
              onInput={() => {
                setIsPaste(false);
                setWrapperOptions();
              }}
              currentLanguage={currentLanguage}
              form={form as HTMLFormElement}
              setCodeView={setCodeView}
              setLabel={setLabel}
            />
          </div>
          <StandardInput
            visible={standardInputVisible}
            setStandardInput={setStandardInput}
          />
        </div>
        <Show when={wrapperOptions()} keyed>
          {(wrapperOptions) => (
            <Output
              codeView={codeView()}
              stdin={standardInput()}
              wrapperOptions={wrapperOptions}
              setWrapperOptions={setWrapperOptions}
            />
          )}
        </Show>
      </div>
    </form>
  );
}
