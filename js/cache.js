export default class Cache {
    constructor(initializer) {
        this.elems = new Map
        this.initializer = initializer
    }

    get(key) {
        if (this.elems.has(key)) {
            return this.elems.get(key)
        }
        const value = this.initializer(key)
        this.elems.set(key, value)
        return value
    }
}
