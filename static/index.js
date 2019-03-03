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
function updateHighlighters() {
    const option = language.selectedOptions[0]
    editor.setOption('mode', option.getAttribute('data-mime'))
    CodeMirror.autoLoadMode(editor, option.getAttribute('data-highlighter-mode'))
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
