import { Wrapper } from './types'

const filterRegex = /(?:\t\.(?:text|file|section|globl|p2align|type|cfi_.*|size|section)\b|.Lfunc_end).*\n?/g

export default class Output {
    output: HTMLDivElement
    filterAsm: HTMLLabelElement
    filterAsmCheckbox: HTMLInputElement
    wrapper: Wrapper
    json: {
        stdout: string,
        stderr: string,
        status: number | null,
    }

    constructor(output) {
        this.output = output
        this.filterAsm = document.createElement('label')
        this.filterAsmCheckbox = document.createElement('input')
        this.filterAsmCheckbox.type = 'checkbox'
        this.filterAsmCheckbox.checked = true
        this.filterAsmCheckbox.addEventListener('change', () => this.update())
        this.filterAsm.append(' ', this.filterAsmCheckbox, ' Filter assembler directives')
    }

    clear() {
        this.output.textContent = ''
    }

    error() {
        this.output.textContent = 'An error occured while running the code. Try again.'
    }

    display(wrapper, json) {
        this.wrapper = wrapper
        this.json = json
        this.update()
    }

    update() {
        const { stdout, stderr, status } = this.json
        this.output.textContent = ''
        if (stderr) {
            const stderrHeader = document.createElement('h2')
            stderrHeader.textContent = 'Standard error'
            const stderrElement = document.createElement('pre')
            stderrElement.textContent = stderr
            this.output.append(stderrHeader, stderrElement)
        }
        if (!this.wrapper.isFormatter) {
            const stdoutHeader = document.createElement('div')
            stdoutHeader.className = 'stdout-header'
            const stdoutHeaderH2 = document.createElement('h2')
            stdoutHeaderH2.textContent = 'Standard output'
            if (status) {
                stdoutHeaderH2.textContent += ` (exit code ${status})`
            }
            stdoutHeader.append(stdoutHeaderH2)
            if (this.wrapper.isAsm) {
                stdoutHeader.append(this.filterAsm)
            }
            const stdoutElement = document.createElement('pre')
            if (stdout) {
                if (this.wrapper.isAsm && this.filterAsmCheckbox.checked) {
                    stdoutElement.textContent = stdout.replace(filterRegex, "")
                } else {
                    stdoutElement.textContent = stdout
                }
            } else {
                const italic = document.createElement('i')
                italic.textContent = '(no output)'
                stdoutElement.append(italic)
            }
            this.output.append(stdoutHeader, stdoutElement)
        }
    }
}
