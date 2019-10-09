import { get, set } from './config'

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
}

export async function getCurrentEditor() {
    const editorType = await get('editor-type')
    return editorType ? editorType.value : 'codemirror'
}

export function setCurrentEditor(newEditor) {
    return set('editor-type', newEditor)
}
