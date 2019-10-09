function promisify(callback) {
    return new Promise((resolve, reject) => {
        const request = callback()
        request.onsuccess = () => resolve(request.result)
        request.onerror = () => reject(request.error)
    })
}

const dbPromise = promisify(() => {
    const request = indexedDB.open('pastebinrun', 1)
    request.onupgradeneeded = ({ oldVersion }) => {
        if (oldVersion < 1) {
            request.result.createObjectStore('config', { keyPath: 'key' })
        }
    }
    return request
})

export async function get(key) {
    const db = await dbPromise
    const tx = db.transaction('config', 'readonly')
    return await promisify(() => tx.objectStore('config').get(key))
}

export async function set(key, value) {
    const db = await dbPromise
    const tx = db.transaction('config', 'readwrite')
    return await promisify(() => tx.objectStore('config').put({ key, value }))
}
