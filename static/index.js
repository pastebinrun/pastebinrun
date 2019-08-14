// Essentially <noscript>, but also working for ancient web browsers
// not supporting ES6 used by this script.
for (const element of document.querySelectorAll('[data-autohide]')) {
    element.style.display = 'none'
}
for (const element of document.querySelectorAll('[data-autodisable]')) {
    element.disabled = true
}

// Code editor things
CodeMirror.modeURL = '/static/mode/%N/%N.js'
const editor = CodeMirror.fromTextArea(document.getElementById('code'), {
    lineNumbers: true,
    matchBrackets: true,
    lineWrapping: true,
    viewportMargin: Infinity,
    minLines: 40,
})
const language = document.getElementById('language')
const futures = new Map()
function fetchLanguage(id) {
    if (futures.has(id)) {
        return futures.get(id)
    }
    const future = fetch(`/api/v0/language/${id}`).then(x => x.json())
    futures.set(id, future)
    return future
}

const wrapperButtons = document.getElementById('wrapper-buttons')
const compilerOptions = document.createElement('input')
compilerOptions.placeholder = 'Compiler options'
const buttons = document.createElement('span')
wrapperButtons.append(compilerOptions, buttons)

const filterAsm = document.createElement('label')
const filterAsmCheckbox = document.createElement('input')
filterAsmCheckbox.type = 'checkbox'
filterAsmCheckbox.checked = true
filterAsm.append(' ', filterAsmCheckbox, ' Filter assembler directives')
const filterRegex = /(?:\t\.(?:text|file|section|globl|p2align|type|cfi_.*|size|section)\b|.Lfunc_end).*\n?/g

let abortEval = new AbortController
async function updateLanguage() {
    const initialValue = language.selectedOptions[0].value
    const { mime, mode, wrappers } = await fetchLanguage(initialValue)
    if (initialValue === language.selectedOptions[0].value) {
        editor.setOption('mode', mime)
        CodeMirror.autoLoadMode(editor, mode)
        buttons.textContent = ''
        compilerOptions.style.display = wrappers.length ? 'inline' : 'none'
        for (const { id, label, isAsm, isFormatter } of wrappers) {
            const button = document.createElement('button')
            button.textContent = label
            button.addEventListener('click', e => {
                e.preventDefault()
                const body = new URLSearchParams
                body.append('compilerOptions', compilerOptions.value)
                body.append('code', editor.getValue())
                abortEval.abort()
                abortEval = new AbortController
                const parameters = {
                    method: 'POST',
                    body,
                    headers: {
                        'Content-Type': 'application/x-www-form-urlencoded',
                    },
                    signal: abortEval.signal,
                }
                const output = document.getElementById('output')
                output.textContent = ''
                fetch(`/api/v0/run/${id}`, parameters)
                    .then(x => x.json())
                    .catch(e => {
                        if (e.name != 'AbortError')
                            output.textContent = 'An error occured while running the code. Try again.'
                    })
                    .then(({ status, stdout, stderr }) => {
                        function updateStdout() {
                            if (stdout) {
                                if (isAsm && filterAsmCheckbox.checked) {
                                    stdoutElement.textContent = stdout.replace(filterRegex, "")
                                } else {
                                    stdoutElement.textContent = stdout
                                }
                            } else {
                                const italic = document.createElement('i')
                                italic.textContent = '(no output)'
                                stdoutElement.append(italic)
                            }
                        }
                        let stdoutElement
                        if (stderr) {
                            const stderrHeader = document.createElement('h2')
                            stderrHeader.textContent = 'Standard error'
                            const stderrElement = document.createElement('pre')
                            stderrElement.textContent = stderr
                            output.append(stderrHeader, stderrElement)
                        }
                        if (isFormatter) {
                            editor.setValue(stdout)
                        } else {
                            const stdoutHeader = document.createElement('div')
                            stdoutHeader.className = 'stdout-header'
                            const stdoutHeaderH2 = document.createElement('h2')
                            stdoutHeaderH2.textContent = 'Standard output'
                            if (status) {
                                stdoutHeaderH2.textContent += ` (exit code ${status})`
                            }
                            stdoutHeader.append(stdoutHeaderH2)
                            if (isAsm) {
                                stdoutHeader.append(filterAsm)
                                filterAsmCheckbox.onchange = updateStdout
                            }
                            stdoutElement = document.createElement('pre')
                            updateStdout()
                            output.append(stdoutHeader, stdoutElement)
                        }
                    })
            })
            buttons.appendChild(button)
        }
    }
}
language.addEventListener('change', updateLanguage)
updateLanguage()

// Restore to the main page state on any changes
language.addEventListener('change', restoreOriginalState)
editor.on('change', restoreOriginalState)
function restoreOriginalState() {
    for (const element of document.querySelectorAll('[data-autohide]')) {
        element.style.display = ''
    }
    for (const element of document.querySelectorAll('[data-autodisable]')) {
        element.disabled = false
    }
    for (const element of document.querySelectorAll('[data-delete-on-modify]')) {
        element.remove()
    }
}
