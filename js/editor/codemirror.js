import CodeMirror from 'codemirror'

const languagesMap = new Map([
    ['c', [() => import('codemirror/mode/clike/clike'), 'text/x-csrc']],
    ['c-plus-plus', [() => import('codemirror/mode/clike/clike'), 'text/x-c++src']],
    ['c-sharp', [() => import('codemirror/mode/clike/clike'), 'text/x-csharp']],
    ['haskell', [() => import('codemirror/mode/haskell/haskell'), 'text/x-haskell']],
    ['html', [() => import('codemirror/mode/htmlmixed/htmlmixed'), 'text/html']],
    ['java', [() => import('codemirror/mode/clike/clike'), 'text/x-java']],
    ['javascript', [() => import('codemirror/mode/javascript/javascript'), 'text/javascript']],
    ['jinja2', [() => import('codemirror/mode/jinja2/jinja2'), 'text/jinja2']],
    ['jsx', [() => import('codemirror/mode/jsx/jsx'), 'text/jsx']],
    ['markdown', [() => import('codemirror/mode/markdown/markdown'), 'text/x-markdown']],
    ['perl', [() => import('codemirror/mode/perl/perl'), 'text/x-perl']],
    ['perl6', [() => { }, 'text/x-perl6']],
    ['php', [() => import('codemirror/mode/php/php'), 'application/x-httpd-php']],
    ['plain-text', [() => { }, 'text/plain']],
    ['postgresql', [() => import('codemirror/mode/sql/sql'), 'text/x-pgsql']],
    ['python2', [() => import('codemirror/mode/python/python'), 'text/x-python']],
    ['python3', [() => import('codemirror/mode/python/python'), 'text/x-python']],
    ['rust', [() => import('codemirror/mode/rust/rust'), 'text/x-rustsrc']],
    ['sh', [() => import('codemirror/mode/shell/shell'), 'text/x-sh']],
    ['sql', [() => import('codemirror/mode/sql/sql'), 'text/x-sql']],
    ['sqlite', [() => import('codemirror/mode/sql/sql'), 'text/x-sqlite']],
    ['typescript', [() => import('codemirror/mode/javascript/javascript'), 'application/typescript']],
    ['typescript-jsx', [() => import('codemirror/mode/jsx/jsx'), 'text/typescript-jsx']],
])

class CodeMirrorEditor {
    constructor(editor) {
        this.editor = editor
    }

    async setLanguage(identifier) {
        this.currentIdentifier = identifier
        const [importFn, mime] = await languagesMap.get(identifier)
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
        matchBrackets: true,
        lineWrapping: true,
        viewportMargin: Infinity,
        minLines: 40,
    })
    editor.on('change', onChange)
    return new CodeMirrorEditor(editor)
}
