import CodeMirror from 'codemirror'

// Essentially <noscript>, but also working for ancient web browsers
// not supporting ES6 used by this script.
for (const element of document.querySelectorAll('[data-autohide]')) {
    element.style.display = 'none'
}
for (const element of document.querySelectorAll('[data-autodisable]')) {
    element.disabled = true
}

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
const selector = document.createElement('span')
const compilerOptions = document.createElement('input')
compilerOptions.placeholder = 'Compiler options'
compilerOptions.style.display = 'none'
const buttons = document.createElement('span')
wrapperButtons.append(selector, compilerOptions, buttons)

const filterAsm = document.createElement('label')
const filterAsmCheckbox = document.createElement('input')
filterAsmCheckbox.type = 'checkbox'
filterAsmCheckbox.checked = true
filterAsm.append(' ', filterAsmCheckbox, ' Filter assembler directives')
const filterRegex = /(?:\t\.(?:text|file|section|globl|p2align|type|cfi_.*|size|section)\b|.Lfunc_end).*\n?/g

let abortEval = new AbortController
async function updateLanguage() {
    const initialValue = language.selectedOptions[0].value
    const { mime, mode, implementations } = await fetchLanguage(initialValue)
    const isCorrectLanguage = () => initialValue === language.selectedOptions[0].value
    if (isCorrectLanguage()) {
        if (mode) {
            import(`codemirror/mode/${mode}/${mode}.js`).then(() => {
                if (isCorrectLanguage()) {
                    editor.setOption('mode', mime)
                }
            })
        }
        selector.textContent = ''
        buttons.textContent = ''
        if (implementations.length > 1) {
            const select = document.createElement('select')
            for (const { label, identifier, wrappers } of implementations) {
                const option = document.createElement('option')
                option.textContent = label
                option.showButtons = () => addButtons(wrappers, buttons, `${initialValue}/${identifier}`)
                select.append(option)
            }
            function updateButtons() {
                select.selectedOptions[0].showButtons()
            }
            select.addEventListener('change', updateButtons)
            updateButtons()
            selector.append(select)
        } else if (implementations.length === 1) {
            addButtons(implementations[0].wrappers, buttons, `${initialValue}/${implementations[0].identifier}`)
        }
        compilerOptions.style.display = implementations.length ? 'inline' : 'none'
    }
}

function addButtons(wrappers, buttons, prefix) {
    buttons.textContent = ''
    for (const { identifier, label, isAsm, isFormatter } of wrappers) {
        const button = document.createElement('button')
        button.textContent = label
        button.addEventListener('click', async e => {
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
            let response
            try {
                response = await (await fetch(`/api/v0/run/${prefix}/${identifier}`, parameters)).json()
            } catch (e) {
                if (e.name != 'AbortError')
                    output.textContent = 'An error occured while running the code. Try again.'
                throw e
            }
            const { status, stdout, stderr } = response
            function updateStdout() {
                stdoutElement.textContent = ''
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
        buttons.appendChild(button)
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
