CodeMirror.modeURL = "/static/mode/%N/%N.js"
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
