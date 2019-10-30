import { getCurrentEditor, setCurrentEditor, types } from '../../editor-types'
import './config-page.css'

export default async function createSettings(node) {
    node.textContent = 'Loading settings\u2026'
    const currentEditor = getCurrentEditor()
    node.textContent = 'Editor type: '
    for (const id in types) {
        const label = document.createElement('label')
        const radio = document.createElement('input')
        radio.type = 'radio'
        radio.name = 'current-editor'
        radio.checked = id === currentEditor
        radio.addEventListener('change', async () => {
            await setCurrentEditor(id)
            const success = document.createElement('p')
            success.className = 'success'
            success.textContent = 'Configuration was updated'
            node.append(success)
            setTimeout(() => {
                success.style.opacity = '0'
                setTimeout(() => {
                    success.remove()
                }, 1 * 1000)
            }, 3 * 1000)
        })
        label.append(radio, ` ${types[id].name}`)
        node.append(label)
    }
}
