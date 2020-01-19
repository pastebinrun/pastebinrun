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

class TextAreaEditor {
    textarea: HTMLTextAreaElement
    onChange: () => void

    constructor(textarea, onChange) {
        this.textarea = textarea
        this.onChange = onChange
    }

    setLanguage() { }

    getValue() {
        return this.textarea.value
    }

    setValue(value) {
        this.textarea.value = value
    }

    update() {}

    unload() {
        this.textarea.removeEventListener('input', this.onChange)
    }
}

export default function createTextareaEditor(textarea, onChange) {
    textarea.addEventListener('input', onChange)
    const textAreaEditor = new TextAreaEditor(textarea, onChange)
    return textAreaEditor
}
