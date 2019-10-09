import addOptionsLink from './views/config-link'

addOptionsLink()

const convertedNodes = [
    { id: 'options', view() { return import('./views/config-page/config-page') } },
    { id: 'editor', view() { return import('./views/editor/editor') } },
]

for (const { id, view } of convertedNodes) {
    const node = document.getElementById(id)
    if (node !== null) {
        view().then(module => module.default(node))
    }
}
