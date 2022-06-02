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

import {
  Accessor,
  createEffect,
  createResource,
  createSignal,
  For,
  Setter,
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
  let previousHelloWorld: string = "";
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
    <>
      {data.loading || (
        <>
          <div class="group">
            <For
              each={
                data()!.implementations[currentImplementationIndex()]?.wrappers
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
          {data()!.implementations.length > 1 && (
            <div class="group">
              <label>
                {"Implementation: "}
                <select
                  onChange={(e) =>
                    setCurrentImplementationIndex(+e.currentTarget.value)
                  }
                >
                  <For each={data()!.implementations}>
                    {({ label }, index) => (
                      <option value={index()}>{label}</option>
                    )}
                  </For>
                </select>
              </label>
            </div>
          )}
          {data()!.implementations.length && (
            <div class="group">
              <label>
                {"Compiler options: "}
                <input ref={compilerOptions!} />
              </label>
            </div>
          )}
        </>
      )}
    </>
  );
}
