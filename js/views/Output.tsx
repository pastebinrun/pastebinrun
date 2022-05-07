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
  createEffect,
  createResource,
  Match,
  onCleanup,
  Setter,
  Switch,
} from "solid-js";
import CodeView from "../models/CodeView";
import WrapperOptions from "../models/WrapperOptions";
import OutputBox from "./OutputBox";
import Spinner from "./Spinner";

export default function Output(props: {
  codeView: CodeView;
  stdin: string;
  wrapperOptions: WrapperOptions;
  setWrapperOptions: Setter<WrapperOptions | undefined>;
}) {
  const abortController = new AbortController();
  const [output, { refetch }] = createResource(async () => {
    const body = new URLSearchParams();
    body.append("compilerOptions", props.wrapperOptions.compilerOptions);
    body.append("code", props.codeView.code);
    body.append("stdin", props.stdin);
    return (
      await fetch(`/api/v0/run/${props.wrapperOptions.wrapper.identifier}`, {
        method: "POST",
        body,
        headers: {
          "Content-Type": "application/x-www-form-urlencoded",
        },
        signal: abortController.signal,
      })
    ).json();
  });
  createEffect(() => {
    if (props.wrapperOptions.wrapper.isFormatter && output()?.status === 0) {
      props.codeView.code = output().output.replace(
        /\x7F(?:E[^\x7F]*|O)?/g,
        ""
      );
      if (!output().output.includes("\x7F")) {
        props.setWrapperOptions();
      }
    }
  });
  onCleanup(() => abortController.abort());
  return (
    <div id="outputcontainer">
      <div id="output">
        <Switch
          fallback={
            <OutputBox
              output={output()}
              wrapper={props.wrapperOptions.wrapper}
            />
          }
        >
          <Match when={output.loading}>
            <Spinner />
          </Match>
          <Match when={output.error}>
            {"An error occured while running the code. "}
            <a
              href="#"
              onClick={(e) => {
                e.preventDefault();
                refetch();
              }}
            >
              Try again
            </a>
            .
          </Match>
        </Switch>
      </div>
    </div>
  );
}
