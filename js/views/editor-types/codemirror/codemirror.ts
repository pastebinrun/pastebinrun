import * as CodeMirror from 'codemirror'
import 'codemirror/lib/codemirror.css'
import './codemirror.css'

const languagesMap = {
    c: [() => import('codemirror/mode/clike/clike'), 'text/x-csrc'],
    cpp: [() => import('codemirror/mode/clike/clike'), 'text/x-c++src'],
    csharp: [() => import('codemirror/mode/clike/clike'), 'text/x-csharp'],
    go: [() => import('codemirror/mode/go/go'), 'text/x-go'],
    haskell: [() => import('codemirror/mode/haskell/haskell'), 'text/x-haskell'],
    html: [() => import('codemirror/mode/htmlmixed/htmlmixed'), 'text/html'],
    java: [() => import('codemirror/mode/clike/clike'), 'text/x-java'],
    javascript: [() => import('codemirror/mode/javascript/javascript'), 'text/javascript'],
    jinja2: [() => import('codemirror/mode/jinja2/jinja2'), 'text/jinja2'],
    jsx: [() => import('codemirror/mode/jsx/jsx'), 'text/jsx'],
    markdown: [() => import('codemirror/mode/markdown/markdown'), 'text/x-markdown'],
    perl: [() => import('codemirror/mode/perl/perl'), 'text/x-perl'],
    php: [() => import('codemirror/mode/php/php'), 'application/x-httpd-php'],
    plaintext: [() => { }, 'text/plain'],
    postgresql: [() => import('codemirror/mode/sql/sql'), 'text/x-pgsql'],
    python2: [() => import('codemirror/mode/python/python'), 'text/x-python'],
    python: [() => import('codemirror/mode/python/python'), 'text/x-python'],
    raku: [() => import('./raku'), 'text/x-raku'],
    rust: [() => import('codemirror/mode/rust/rust'), 'text/x-rustsrc'],
    sh: [() => import('codemirror/mode/shell/shell'), 'text/x-sh'],
    sql: [() => import('codemirror/mode/sql/sql'), 'text/x-sql'],
    sqlite: [() => import('codemirror/mode/sql/sql'), 'text/x-sqlite'],
    typescript: [() => import('codemirror/mode/javascript/javascript'), 'application/typescript'],
    tsx: [() => import('codemirror/mode/jsx/jsx'), 'text/typescript-jsx'],
}

class CodeMirrorEditor {
    editor: CodeMirror.EditorFromTextArea
    currentIdentifier: string | null = null

    constructor(editor) {
        this.editor = editor
    }

    async setLanguage(identifier) {
        this.currentIdentifier = identifier
        const [importFn, mime] = languagesMap[identifier]
        this.editor.setOption('mode', mime)
        await importFn()
        if (this.currentIdentifier === identifier) {
            this.editor.setOption('mode', mime)
        }
    }

    getValue() {
        return this.editor.getValue()
    }

    setValue(value) {
        this.editor.setValue(value)
    }

    unload() {
        this.editor.toTextArea()
    }
}

export default function createTextareaEditor(textarea, onChange) {
    const editor = CodeMirror.fromTextArea(textarea, {
        lineNumbers: true,
        lineWrapping: true,
    })
    editor.on('change', onChange)
    return new CodeMirrorEditor(editor)
}
