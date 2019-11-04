import addOptionsLink from './views/config-link'
import createSettings from './views/config-page/config-page'
import createEditor from './views/editor/editor'

declare var __webpack_nonce__: string
__webpack_nonce__ = document.getElementsByTagName('link')[0].getAttribute('nonce')

addOptionsLink()

const convertedNodes = [
    { id: 'options', view: createSettings },
    { id: 'editor', view: createEditor },
]

for (const { id, view } of convertedNodes) {
    const node = document.getElementById(id)
    if (node !== null) {
        view(node)
    }
}

if (document.querySelector('[class*=language-]')) {
    import('./highlight-all')
}
