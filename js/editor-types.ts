export interface EditorType {
    setLanguage(identifier: string): void
    getValue(): string
    setValue(text: string): void
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
