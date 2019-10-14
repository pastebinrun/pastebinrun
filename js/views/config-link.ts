export default function addOptionsLink() {
    const ul = document.createElement('ul')
    const li = document.createElement('li')
    const a = document.createElement('a')
    a.textContent = 'Config'
    a.href = '/config'
    li.append(a)
    ul.append(li)
    document.getElementById('menu-buttons').append(ul)
}
