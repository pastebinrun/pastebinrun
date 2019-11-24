import * as monaco from 'monaco-editor'
import './monaco.css'

declare global {
    interface Window {
        MonacoEnvironment: {
            getWorkerUrl(moduleId: string, label: string): string
        }
    }
}

function getWorkerName(label) {
    if (label === 'json' || label === 'css' || label === 'html') {
        return label
    }
    if (label === 'typescript' || label === 'javascript') {
        return 'typescript'
    }
    return 'editor'
}

self.MonacoEnvironment = {
    getWorkerUrl(_moduleId, label) {
        return `/static/js/${getWorkerName(label)}.worker.js`
    }
}

const languageMap = {
    c: 'objective-c', // Somehow the repo doesn't have C language
    cpp: 'cpp',
    csharp: 'csharp',
    go: 'go',
    haskell: null,
    html: 'html',
    java: 'java',
    javascript: 'javascript',
    jinja2: null,
    jsx: 'javascript',
    markdown: 'markdown',
    perl: 'perl',
    php: 'php',
    plaintext: null,
    postgresql: 'sql',
    python2: 'python',
    python3: 'python',
    raku: null,
    rust: 'rust',
    sh: 'shell',
    sql: 'sql',
    sqlite: 'sql',
    typescript: 'typescript',
    tsx: 'typescript',
}

class MonacoEditor {
    textarea: HTMLTextAreaElement
    container: HTMLDivElement
    editor: monaco.editor.IStandaloneCodeEditor

    constructor(textarea, container, editor) {
        this.textarea = textarea
        this.container = container
        this.editor = editor
    }

    setLanguage(identifier) {
        monaco.editor.setModelLanguage(this.editor.getModel(), languageMap[identifier])
    }

    getValue() {
        return this.editor.getValue()
    }

    setValue(value) {
        this.editor.setValue(value)
    }

    unload() {
        this.textarea.value = this.getValue()
        this.editor.dispose()
        this.container.remove()
        this.textarea.style.display = 'inline'
    }
}

export default function createMonacoEditor(textarea, onChange) {
    const container = document.createElement('div')
    container.className = 'monaco'
    textarea.before(container)
    textarea.style.display = 'none'
    const editor = monaco.editor.create(container, {
        value: textarea.value,
    })
    editor.onDidChangeModelContent(onChange)
    textarea.form.addEventListener('submit', () => textarea.value = editor.getValue())
    return new MonacoEditor(textarea, container, editor)
}
