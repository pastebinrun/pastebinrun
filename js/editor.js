import getLanguage from './get-language'
import Output from './output'
import WrapperButtons from './wrapper-buttons'

class Editor {
    async initialize(form) {
        this.languageSelector = form.querySelector('#language')
        this.wrapperButtons = new WrapperButtons(form.querySelector('#wrapper-buttons'), this.run.bind(this))
        this.editor = (async () => {
            const module = await import('./codemirror-editor')
            return module.default(form.querySelector('#code'), () => this.changeToLookLikeNewPaste())
        })()
        this.output = new Output(output)
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
        const isStillValid = () => identifier === this.getLanguageIdentifier()
        const language = await getLanguage(identifier, isStillValid)
        // This deals with user changing the language after asynchronous event
        if (isStillValid()) {
            this.wrapperButtons.update(language.implementations)
            const editor = await this.editor
            if (isStillValid()) {
                editor.setLanguage(language)
            }
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
        const editor = await this.editor
        body.append('code', editor.getValue())
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
            editor.setValue(response.stdout)
        }
        this.output.display(wrapper, response)
    }
}

export default function createEditor(form) {
    return new Editor().initialize(form)
}
