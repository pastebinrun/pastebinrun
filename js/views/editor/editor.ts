// pastebin.run
// Copyright (C) 2020 Konrad Borowski
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

import createTextareaEditor from '../editor-types/textarea'
import getLanguage from './get-language'
import Output from './output'
import WrapperButtons from './wrapper-buttons'
import { EditorType, types, getCurrentEditor, onChange } from '../../editor-types'

class Editor {
    languageSelector: HTMLSelectElement
    wrapperButtons: WrapperButtons
    codeElement: HTMLTextAreaElement
    output: Output
    autodeleteText: HTMLElement[]
    submitButtons: HTMLInputElement[]
    detailsElement: HTMLDetailsElement
    stdinElement: HTMLTextAreaElement
    editor: EditorType
    currentLanguage: string | null = null
    abortEval: AbortController | null = null
    isHelloWorld: boolean = false

    initialize(form) {
        this.languageSelector = form.querySelector('#language')
        this.wrapperButtons = new WrapperButtons(form.querySelector('#wrapper-buttons'), this.run.bind(this))
        this.codeElement = form.querySelector('#code')
        this.initializeEditor(createTextareaEditor)
        onChange(editor => this.changeEditor(editor))
        this.initConfiguredEditor()
        this.output = Output.addTo(form.querySelector('#split'))
        const output = document.querySelector<HTMLInputElement>('#dboutput')
        if (output) {
            this.displayOutput({}, {
                output: output.value,
                status: +document.querySelector<HTMLInputElement>('#dbstatus')?.value,
            })
        }
        this.autodeleteText = form.querySelectorAll('.autodelete-text')
        this.submitButtons = form.querySelectorAll('[type=submit]')
        for (const submit of this.submitButtons) {
            submit.disabled = true
        }
        form.addEventListener('submit', () => {
            if (this.output.json && !this.output.wrapper.isFormatter) {
                for (const name of ['output', 'status']) {
                    const elem = form.querySelector(`[name=${name}]`) || document.createElement('input')
                    elem.type = 'hidden'
                    elem.name = name
                    elem.value = this.output.json[name]
                    form.append(elem)
                }
            } else {
                this.stdinElement.value = ''
            }
        })
        this.detailsElement = document.createElement('details')
        const summary = document.createElement('summary')
        summary.textContent = 'Standard input'
        this.stdinElement = document.createElement('textarea')
        this.stdinElement.name = 'stdin'
        this.stdinElement.addEventListener('change', () => {
            this.isHelloWorld = false
            this.changeToLookLikeNewPaste()
        })
        this.detailsElement.append(summary, this.stdinElement)
        const dbStdin = document.querySelector<HTMLInputElement>('#dbstdin')?.value
        if (dbStdin) {
            this.stdinElement.value = dbStdin
            this.detailsElement.open = true
        } else {
            this.detailsElement.style.display = 'none'
        }
        form.querySelector('#extrafields').append(this.detailsElement)
        this.assignEvents()
        this.updateLanguage()
    }

    async initConfiguredEditor() {
        this.changeEditor(await types[getCurrentEditor()].createView())
    }

    changeEditor(createEditor) {
        this.editor.unload()
        this.initializeEditor(createEditor)
    }

    initializeEditor(createEditor) {
        this.editor = createEditor(this.codeElement, () => {
            this.changeToLookLikeNewPaste()
            this.isHelloWorld = false
        })
        if (this.currentLanguage) {
            this.editor.setLanguage(this.currentLanguage)
        }
    }

    setLanguage(language) {
        this.currentLanguage = language
        this.editor.setLanguage(language)
    }

    changeToLookLikeNewPaste() {
        this.output.clear()
        this.editor.update()
        for (const element of this.autodeleteText) {
            element.style.display = 'none'
        }
        for (const submit of this.submitButtons) {
            submit.disabled = false
        }
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
            this.detailsElement.style.display = language.implementations.length ? 'block' : 'none'
            this.wrapperButtons.update(language.implementations)
            if (this.isHelloWorld || this.editor.getValue() === '') {
                this.editor.setValue(language.helloWorld)
                this.isHelloWorld = true
            }
        }
    }

    getLanguageIdentifier() {
        return this.languageSelector.selectedOptions[0].value
    }

    async run(wrapper, compilerOptions) {
        this.output.spin()
        this.editor.update()
        if (this.abortEval) {
            this.abortEval.abort()
        }
        this.abortEval = new AbortController
        const body = new URLSearchParams
        body.append('compilerOptions', compilerOptions)
        body.append('code', this.editor.getValue())
        body.append('stdin', this.stdinElement.value)
        const parameters = {
            method: 'POST',
            body,
            headers: {
                'Content-Type': 'application/x-www-form-urlencoded',
            },
            signal: this.abortEval.signal,
        }
        const path = `/api/v0/run/${wrapper.identifier}`
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
            this.editor.setValue(response.output.replace(/\x7FE[^]*?(?:\x7FO|$)/g, ""))
        }
        this.displayOutput(wrapper, response)
    }

    displayOutput(wrapper, response) {
        this.output.display(wrapper, response)
        this.editor.update()
    }
}

export default function createEditor(form) {
    new Editor().initialize(form)
}
