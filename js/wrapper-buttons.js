export default class WrapperButtons {
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
            option.identifier = identifier
            option.wrappers = wrappers
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
            for (const wrapper of option.wrappers) {
                const button = document.createElement('button')
                button.textContent = wrapper.label
                button.addEventListener('click', e => {
                    e.preventDefault()
                    this.run(option.identifier, wrapper, this.compilerOptions.value)
                })
                this.buttons.append(button)
            }
        }
    }
}