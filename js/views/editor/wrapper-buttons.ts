import { Wrapper } from './types'

export default class WrapperButtons {
    buttonsContainer: HTMLSpanElement
    compilerOptions: HTMLInputElement
    buttons: HTMLSpanElement
    run: (wrapper: Wrapper, compilerOptions: string) => void
    abortController: AbortController
    select: HTMLSelectElement
    optionMap = new WeakMap<HTMLOptionElement, { identifier: string, wrappers: Wrapper[] }>()
    globalKeyEvent: (e: KeyboardEvent) => void | null = null

    constructor(buttonsContainer, run) {
        this.buttonsContainer = buttonsContainer
        this.compilerOptions = document.createElement('input')
        this.compilerOptions.placeholder = 'Compiler options'
        this.buttons = document.createElement('span')
        this.run = run
        this.abortController = null
    }

    update(implementations) {
        this.clear()
        this.select = document.createElement('select')
        for (const { label, identifier, wrappers } of implementations) {
            const option = document.createElement('option')
            option.textContent = label
            this.optionMap.set(option, { identifier, wrappers })
            this.select.append(option)
        }
        this.buttonsContainer.textContent = ''
        if (implementations.length > 1) {
            this.buttonsContainer.append(this.select)
            this.select.addEventListener('change', () => this.updateButtons())
        }
        if (implementations.length !== 0) {
            this.buttonsContainer.append(this.compilerOptions, this.buttons)
        }
        this.updateButtons()
    }

    clear() {
        document.removeEventListener('keydown', this.globalKeyEvent)
        this.globalKeyEvent = null
        this.buttonsContainer.textContent = ''
    }

    updateButtons() {
        this.buttons.textContent = ''
        let options = this.select.selectedOptions
        if (options.length === 0) {
            options = this.select.options
        }
        if (options.length !== 0) {
            const option = options[0]
            let first = true
            for (const wrapper of this.optionMap.get(option).wrappers) {
                const button = document.createElement('button')
                button.textContent = wrapper.label
                const event = (e: Event) => {
                    e.preventDefault()
                    this.run(wrapper, this.compilerOptions.value)
                }
                if (first) {
                    this.globalKeyEvent = e => {
                        if (e.key === 'Enter' && (e.ctrlKey || e.metaKey)) {
                            event(e)
                        }
                    }
                    document.addEventListener('keydown', this.globalKeyEvent)
                    first = false
                }
                button.addEventListener('click', event)
                this.buttons.append(button)
            }
        }
    }
}
