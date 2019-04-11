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
async function updateHighlighters() {
    const initialValue = language.selectedOptions[0].value
    const { mime, mode } = await fetchLanguage(initialValue)
    if (initialValue === language.selectedOptions[0].value) {
        editor.setOption('mode', mime)
        CodeMirror.autoLoadMode(editor, mode)
    }
}
language.addEventListener('change', updateHighlighters)
updateHighlighters()

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
