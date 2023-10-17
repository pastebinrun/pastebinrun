// SPDX-FileCopyrightText: 2022 - 2023 Konrad Borowski <konrad@borowski.pw>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

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
    languages.selectedOptions[0]?.value,
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
          <div>{label()}</div>
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
