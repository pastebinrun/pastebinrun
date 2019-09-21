import CodeMirror from 'codemirror'

class CodeMirrorEditor {
    constructor(editor) {
        this.editor = editor
    }

    async setLanguage({ mode, mime }) {
        this.currentMime = mime
        this.editor.setOption('mode', mime)
        if (mode) {
            await import(`codemirror/mode/${mode}/${mode}.js`)
            if (this.currentMime === mime) {
                this.editor.setOption('mode', mime)
            }
        }
    }

    getValue() {
        return this.editor.getValue()
    }

    setValue(value) {
        this.editor.setValue(value)
    }

    onChange(callback) {
        this.editor.on('change', callback)
    }
}

export default function createEditor(textarea, onChange) {
    const editor = CodeMirror.fromTextArea(textarea, {
        lineNumbers: true,
        matchBrackets: true,
        lineWrapping: true,
        viewportMargin: Infinity,
        minLines: 40,
    })
    editor.on('change', onChange)
    return new CodeMirrorEditor(editor)
}
