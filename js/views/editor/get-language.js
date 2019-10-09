const cache = new Map

async function fetchLanguage(identifier) {
    const response = await fetch(`/api/v0/language/${identifier}`)
    return await response.json()
}

export default async function getLanguage(identifier, shouldRetry) {
    if (cache.has(identifier)) {
        return cache.get(identifier)
    }
    while (shouldRetry()) {
        try {
            const response = await fetchLanguage(identifier)
            cache.set(identifier, response)
            return response
        } catch (e) {
            await new Promise(resolve => setTimeout(resolve, 1000))
        }
    }
    await new Promise(() => { })
}
