import createTextareaEditor from '../editor-types/textarea'
import getLanguage from './get-language'
import Output from './output'
import WrapperButtons from './wrapper-buttons'
import { EditorType, types, getCurrentEditor } from '../../editor-types'

class Editor {
    languageSelector: HTMLSelectElement
    wrapperButtons: WrapperButtons
    codeElement: HTMLTextAreaElement
    output: Output
    autodeleteText: HTMLSpanElement
    autodeleteCheckbox: HTMLLabelElement
    submit: HTMLInputElement
    editor: EditorType
    currentLanguage: string | null = null
    abortEval: AbortController | null = null

    async initialize(form) {
        this.languageSelector = form.querySelector('#language')
        this.wrapperButtons = new WrapperButtons(form.querySelector('#wrapper-buttons'), this.run.bind(this))
        this.codeElement = form.querySelector('#code')
        this.initializeEditor(createTextareaEditor)
        this.initConfiguredEditor()
        this.output = new Output(form.querySelector('#output'))
        this.autodeleteText = form.querySelector('#autodelete-text')
        this.autodeleteCheckbox = form.querySelector('#automatically-hidden-label')
        this.submit = form.querySelector('[type=submit]')
        this.submit.disabled = true
        if (this.autodeleteText) {
            this.autodeleteCheckbox.style.display = 'none'
        }
        this.assignEvents()
        this.updateLanguage()
    }

    async initConfiguredEditor() {
        this.changeEditor(await types[await getCurrentEditor()].createView())
    }

    changeEditor(createEditor) {
        this.editor.unload()
        this.initializeEditor(createEditor)
    }

    initializeEditor(createEditor) {
        this.editor = createEditor(this.codeElement, () => this.changeToLookLikeNewPaste())
        if (this.currentLanguage) {
            this.editor.setLanguage(this.currentLanguage)
        }
    }

    setLanguage(language) {
        this.currentLanguage = language
        this.editor.setLanguage(language)
    }

    changeToLookLikeNewPaste() {
        if (this.autodeleteText) {
            this.autodeleteText.style.display = 'none'
            this.autodeleteCheckbox.style.display = ''
        }
        this.submit.disabled = false
    }

    assignEvents() {
        this.languageSelector.addEventListener('change', () => {
            this.updateLanguage()
            this.changeToLookLikeNewPaste()
        })
    }

    async updateLanguage() {
        this.wrapperButtons.clear()
        const identifier = this.getLanguageIdentifier()
        this.setLanguage(identifier)
        const isStillValid = () => identifier === this.getLanguageIdentifier()
        const language = await getLanguage(identifier, isStillValid)
        // This deals with user changing the language after asynchronous event
        if (isStillValid()) {
            this.wrapperButtons.update(language.implementations)
        }
    }

    getLanguageIdentifier() {
        return this.languageSelector.selectedOptions[0].value
    }

    async run(implementationIdentifier, wrapper, compilerOptions) {
        this.output.clear()
        if (this.abortEval) {
            this.abortEval.abort()
        }
        this.abortEval = new AbortController
        const body = new URLSearchParams
        body.append('compilerOptions', compilerOptions)
        body.append('code', this.editor.getValue())
        const parameters = {
            method: 'POST',
            body,
            headers: {
                'Content-Type': 'application/x-www-form-urlencoded',
            },
            signal: this.abortEval.signal,
        }
        const languageIdentifier = this.getLanguageIdentifier()
        const path = `/api/v0/run/${languageIdentifier}/${implementationIdentifier}/${wrapper.identifier}`
        let response
        try {
            response = await (await fetch(path, parameters)).json()
        } catch (e) {
            if (e.name === 'AbortError') {
                return
            }
            this.output.error()
            throw e
        }
        if (wrapper.isFormatter) {
            this.editor.setValue(response.stdout)
        }
        this.output.display(wrapper, response)
    }
}

export default function createEditor(form) {
    return new Editor().initialize(form)
}
