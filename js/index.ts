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

import addOptionsLink from './views/config-link'
import createSettings from './views/config-page/config-page'
import createEditor from './views/editor/editor'

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
