import { markdown } from "@codemirror/lang-markdown";
import { LanguageDescription } from "@codemirror/language";
import { languages } from "@codemirror/language-data";

export default markdown({
  codeLanguages: languages.concat(
    LanguageDescription.of({
      name: "Nix",
      extensions: ["nix"],
      async load() {
        return (await import("@replit/codemirror-lang-nix")).nix();
      },
    }),
  ),
});
