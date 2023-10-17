// SPDX-FileCopyrightText: 2022 - 2023 Konrad Borowski <konrad@borowski.pw>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import {
  Accessor,
  createEffect,
  createResource,
  createSignal,
  For,
  Setter,
  Show,
} from "solid-js";
import CodeView from "../models/CodeView";
import Wrapper from "../models/Wrapper";
import WrapperBarButton from "./WrapperBarButton";

type Language = {
  helloWorld: string;
  implementations: {
    label: string;
    wrappers: Wrapper[];
  }[];
};

async function fetchLanguage(language: string): Promise<Language> {
  return (await fetch(`/api/v0/language/${language}`)).json();
}

export default function WrapperBar({
  currentLanguage,
  setStandardInputVisible,
  codeView,
  runEvaluation,
}: {
  currentLanguage: Accessor<string>;
  setStandardInputVisible: Setter<boolean>;
  codeView: Accessor<CodeView>;
  runEvaluation: (wrapper: Wrapper, compilerOptions: string) => void;
}) {
  const [data] = createResource(currentLanguage, fetchLanguage);
  const [currentImplementationIndex, setCurrentImplementationIndex] =
    createSignal(0);
  let previousHelloWorld = "";
  createEffect(() => {
    const loadedData = data();
    if (loadedData) {
      setStandardInputVisible(loadedData.implementations.length > 0);
      const code = codeView().code;
      if (!code || code === previousHelloWorld) {
        previousHelloWorld = codeView().code = loadedData.helloWorld;
      } else {
        previousHelloWorld = "";
      }
    }
  });
  let compilerOptions: HTMLInputElement;
  return (
    <Show when={data()} keyed>
      {(data) => (
        <>
          <div class="group">
            <For
              each={
                data.implementations[currentImplementationIndex()]?.wrappers
              }
            >
              {(wrapper, index) => (
                <>
                  {" "}
                  <WrapperBarButton
                    index={index()}
                    runEvaluation={() => {
                      runEvaluation(wrapper, compilerOptions.value);
                    }}
                  >
                    {wrapper.label}
                  </WrapperBarButton>
                </>
              )}
            </For>
          </div>
          <Show when={data.implementations.length > 1}>
            <div class="group">
              <label>
                {"Implementation: "}
                <select
                  onChange={(e) =>
                    setCurrentImplementationIndex(+e.currentTarget.value)
                  }
                >
                  <For each={data.implementations}>
                    {({ label }, index) => (
                      <option value={index()}>{label}</option>
                    )}
                  </For>
                </select>
              </label>
            </div>
          </Show>
          <Show when={data.implementations.length}>
            <div class="group">
              <label>
                {"Compiler options: "}
                <input ref={(e) => (compilerOptions = e)} />
              </label>
            </div>
          </Show>
        </>
      )}
    </Show>
  );
}
