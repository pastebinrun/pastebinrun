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

export interface EditorType {
    setLanguage(identifier: string): void
    getValue(): string
    setValue(text: string): void
    update(): void
    unload(): void
}

export const types = {
    textarea: {
        name: 'Textarea',
        async createView() {
            return (await import('./views/editor-types/textarea')).default
        },
    },
    codemirror: {
        name: 'CodeMirror',
        async createView() {
            return (await import('./views/editor-types/codemirror/codemirror')).default
        },
    },
    monaco: {
        name: 'Monaco (Visual Studio Code)',
        async createView() {
            return (await import('./views/editor-types/monaco/monaco')).default
        }
    },
}

export function getCurrentEditor() {
    return localStorage.getItem('editorType') || 'codemirror'
}

export function setCurrentEditor(newEditor) {
    return localStorage.setItem('editorType', newEditor)
}

export function onChange(callback: (createEditor: (textArea: HTMLTextAreaElement, onChange: () => void) => EditorType) => void) {
    addEventListener('storage', async ({ key, newValue }) => {
        if (key === 'editorType') {
            callback(await types[newValue].createView())
        }
    })
}
