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
  async nix() {
    return (await import("@replit/codemirror-lang-nix")).nix();
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

export default languagesMap;
