class TextAreaEditor {
    textarea: HTMLTextAreaElement
    onChange: () => void

    constructor(textarea, onChange) {
        this.textarea = textarea
        this.onChange = onChange
    }

    setLanguage() { }

    getValue() {
        return this.textarea.value
    }

    setValue(value) {
        this.textarea.value = value
    }

    update() {}

    unload() {
        this.textarea.removeEventListener('input', this.onChange)
    }
}

export default function createTextareaEditor(textarea, onChange) {
    textarea.addEventListener('input', onChange)
    const textAreaEditor = new TextAreaEditor(textarea, onChange)
    return textAreaEditor
}
