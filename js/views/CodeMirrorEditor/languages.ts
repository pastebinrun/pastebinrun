// SPDX-FileCopyrightText: 2022 - 2023 Konrad Borowski <konrad@borowski.pw>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import { StreamLanguage } from "@codemirror/language";
import { Extension } from "@codemirror/state";

const languagesMap: { [name: string]: () => Promise<Extension> } = {
  async c() {
    return (await import("@codemirror/lang-cpp")).cpp();
  },
  async cpp() {
    return (await import("@codemirror/lang-cpp")).cpp();
  },
  async csharp() {
    return StreamLanguage.define(
      (await import("@codemirror/legacy-modes/mode/clike")).csharp,
    );
  },
  async go() {
    return StreamLanguage.define(
      (await import("@codemirror/legacy-modes/mode/go")).go,
    );
  },
  async haskell() {
    return StreamLanguage.define(
      (await import("@codemirror/legacy-modes/mode/haskell")).haskell,
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
      (await import("@codemirror/legacy-modes/mode/jinja2")).jinja2,
    );
  },
  async jsx() {
    return (await import("@codemirror/lang-javascript")).javascript({
      jsx: true,
    });
  },
  async markdown() {
    return (await import("./markdown")).default;
  },
  async nix() {
    return (await import("@replit/codemirror-lang-nix")).nix();
  },
  async perl() {
    return StreamLanguage.define(
      (await import("@codemirror/legacy-modes/mode/perl")).perl,
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
      (await import("@codemirror/legacy-modes/mode/shell")).shell,
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

export default languagesMap;
