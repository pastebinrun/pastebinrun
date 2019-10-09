class TextAreaEditor {
    constructor(textarea, onChange) {
        this.textarea = textarea
        this.onChange = onChange
    }

    setLanguage() {}

    getValue() {
        return this.textarea.value
    }

    setValue(value) {
        this.textarea.value = value
    }

    unload() {
        this.textarea.removeEventListener('change', this.onChange)
    }
}

export default function createTextareaEditor(textarea, onChange) {
    textarea.addEventListener('change', onChange)
    const textAreaEditor = new TextAreaEditor(textarea, onChange)
    return textAreaEditor
}
