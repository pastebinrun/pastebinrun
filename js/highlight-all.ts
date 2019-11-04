import * as Prism from 'prismjs'
import 'prismjs/plugins/autoloader/prism-autoloader'
import 'prismjs/themes/prism.css'

Prism.plugins.autoloader.languages_path = 'https://cdnjs.cloudflare.com/ajax/libs/prism/1.17.1/components/'
Prism.highlightAll()
