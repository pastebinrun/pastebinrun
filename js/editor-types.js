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

export async function getCurrentEditor() {
    return sessionStorage.getItem('editorType') || 'codemirror'
}

export function setCurrentEditor(newEditor) {
    return sessionStorage.setItem('editorType', newEditor)
}
