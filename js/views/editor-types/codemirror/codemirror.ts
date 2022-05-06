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

import { EditorView, EditorState, basicSetup } from "@codemirror/basic-setup";
import { indentWithTab } from "@codemirror/commands";
import { indentUnit, StreamLanguage } from "@codemirror/language";
import { Compartment, Extension } from "@codemirror/state";
import { keymap } from "@codemirror/view";
import { getTabIndentationConfiguration } from "../../../editor-types";
import "./codemirror.css";

const tabIndentation = new Compartment();

const languagesMap: { [name: string]: () => Promise<Extension> } = {
  async c() {
    return (await import("@codemirror/lang-cpp")).cpp();
  },
  async cpp() {
    return (await import("@codemirror/lang-cpp")).cpp();
  },
  async csharp() {
    return StreamLanguage.define(
      (await import("@codemirror/legacy-modes/mode/clike")).csharp
    );
  },
  async go() {
    return StreamLanguage.define(
      (await import("@codemirror/legacy-modes/mode/go")).go
    );
  },
  async haskell() {
    return StreamLanguage.define(
      (await import("@codemirror/legacy-modes/mode/haskell")).haskell
    );
  },
  async html() {
    return (await import("@codemirror/lang-html")).html();
  },
  async java() {
    return (await import("@codemirror/lang-java")).java();
  },
  async javascript() {
    return (await import("@codemirror/lang-javascript")).javascript();
  },
  async jinja2() {
    return StreamLanguage.define(
      (await import("@codemirror/legacy-modes/mode/jinja2")).jinja2
    );
  },
  async jsx() {
    return (await import("@codemirror/lang-javascript")).javascript({
      jsx: true,
    });
  },
  async markdown() {
    return (await import("@codemirror/lang-markdown")).markdown();
  },
  async perl() {
    return StreamLanguage.define(
      (await import("@codemirror/legacy-modes/mode/perl")).perl
    );
  },
  async php() {
    return (await import("@codemirror/lang-php")).php();
  },
  async postgresql() {
    const { sql, PostgreSQL } = await import("@codemirror/lang-sql");
    return sql({ dialect: PostgreSQL });
  },
  async python() {
    return (await import("@codemirror/lang-python")).python();
  },
  async rust() {
    return (await import("@codemirror/lang-rust")).rust();
  },
  async sh() {
    return StreamLanguage.define(
      (await import("@codemirror/legacy-modes/mode/shell")).shell
    );
  },
  async sql() {
    return (await import("@codemirror/lang-sql")).sql();
  },
  async sqlite() {
    const { sql, SQLite } = await import("@codemirror/lang-sql");
    return sql({ dialect: SQLite });
  },
  async typescript() {
    return (await import("@codemirror/lang-javascript")).javascript({
      typescript: true,
    });
  },
  async tsx() {
    return (await import("@codemirror/lang-javascript")).javascript({
      jsx: true,
      typescript: true,
    });
  },
};

function getTabIndentationExtension(value: boolean) {
  return value ? keymap.of([indentWithTab]) : [];
}

class CodeMirrorEditor {
  tabIndentation: Compartment;
  language: Compartment;
  view: EditorView;
  textarea: HTMLTextAreaElement;
  storageListener: (e: StorageEvent) => void;
  submitListener: () => void;
  currentIdentifier: string | null = null;

  constructor(
    tabIndentation: Compartment,
    language: Compartment,
    editor: EditorView,
    textarea: HTMLTextAreaElement,
    submitListener: () => void
  ) {
    this.tabIndentation = tabIndentation;
    this.language = language;
    this.view = editor;
    this.textarea = textarea;
    this.storageListener = ({ key, newValue }) => {
      if (key !== "tabIndentation") {
        return;
      }
      this.view.dispatch({
        effects: this.tabIndentation.reconfigure(
          getTabIndentationExtension(newValue)
        ),
      });
    };
    addEventListener("storage", this.storageListener);
    this.submitListener = submitListener;
  }

  async setLanguage(identifier: string) {
    this.currentIdentifier = identifier;
    const callback = languagesMap[identifier];
    const extension = callback ? await callback() : [];
    if (this.currentIdentifier === identifier) {
      this.view.dispatch({ effects: this.language.reconfigure(extension) });
    }
  }

  getValue() {
    return this.view.state.doc.toString();
  }

  setValue(value: string) {
    this.view.dispatch({
      changes: { from: 0, to: this.view.state.doc.length, insert: value },
    });
  }

  update() {
    dispatchEvent(new Event("resize"));
  }

  unload() {
    this.textarea.value = this.getValue();
    this.textarea.style.display = "";
    this.textarea.form.removeEventListener("submit", this.submitListener);
    removeEventListener("storage", this.storageListener);
    this.view.destroy();
  }
}

export default function createTextareaEditor(
  textarea: HTMLTextAreaElement,
  onChange: () => void
) {
  const language = new Compartment();
  let view = new EditorView({
    state: EditorState.create({
      doc: textarea.value,
      extensions: [
        tabIndentation.of(
          getTabIndentationExtension(getTabIndentationConfiguration())
        ),
        keymap.of([{ key: "Ctrl-Enter", run: () => true }]),
        basicSetup,
        EditorView.updateListener.of((v) => {
          if (v.docChanged) {
            onChange();
          }
        }),
        EditorView.lineWrapping,
        indentUnit.of(" ".repeat(4)),
        language.of([]),
      ],
    }),
  });
  textarea.parentNode.insertBefore(view.dom, textarea);
  textarea.style.display = "none";
  const submitListener = () => (textarea.value = editor.getValue());
  textarea.form.addEventListener("submit", submitListener);
  const editor = new CodeMirrorEditor(
    tabIndentation,
    language,
    view,
    textarea,
    submitListener
  );
  return editor;
}
