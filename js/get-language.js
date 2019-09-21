import Cache from './cache'

const languageCache = new Cache(async identifier => {
    const response = await fetch(`/api/v0/language/${identifier}`)
    return await response.json()
})

export default function getLanguage(identifier) {
    return languageCache.get(identifier)
}
