export default function addOptionsLink() {
    const li = document.createElement('li')
    const a = document.createElement('a')
    a.textContent = 'Config'
    a.href = '/config'
    li.append(a)
    document.querySelector('#menu-buttons ul').append(li)
}
