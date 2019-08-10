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

let abortEval = new AbortController

async function updateLanguage() {
    const initialValue = language.selectedOptions[0].value
    const { mime, mode, wrappers } = await fetchLanguage(initialValue)
    if (initialValue === language.selectedOptions[0].value) {
        editor.setOption('mode', mime)
        CodeMirror.autoLoadMode(editor, mode)
        const buttons = document.getElementById('wrapper-buttons')
        buttons.textContent = ''
        for (const { id, label } of wrappers) {
            const button = document.createElement('button')
            button.textContent = label
            button.addEventListener('click', e => {
                e.preventDefault()
                const body = new URLSearchParams
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
                    .then(({ status, stdout, stderr }) => {
                        if (stderr) {
                            const stderrHeader = document.createElement('h2')
                            stderrHeader.textContent = 'Standard error'
                            const stderrElement = document.createElement('pre')
                            stderrElement.textContent = stderr
                            output.append(stderrHeader, stderrElement)
                        }
                        const stdoutHeader = document.createElement('h2')
                        stdoutHeader.textContent = 'Standard output'
                        if (status) {
                            stdoutHeader.textContent += ` (exit code ${status})`
                        }
                        const stdoutElement = document.createElement('pre')
                        if (stdout) {
                            stdoutElement.textContent = stdout
                        } else {
                            const italic = document.createElement('i')
                            italic.textContent = '(no output)'
                            stdoutElement.append(italic)
                        }
                        output.append(stdoutHeader, stdoutElement)
                    })
                    .catch(e => {
                        if (e.name != 'AbortError')
                            output.textContent = 'An error occured while running the code. Try again.'
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
