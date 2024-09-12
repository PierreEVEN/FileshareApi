class EncString {

    /**
     * @type {string}
     * @private
     */
    constructor(data) {
        /**
         * @type {string}
         * @private
         */
        this.__encoded = data;
    }

    /**
     * @returns {string}
     */
    plain() {
        return decodeURI(this.__encoded)
    }

    /**
     * @returns {string}
     */
    encoded() {
        return this.__encoded;
    }

    /**
     * @param raw_string {String}
     * @returns {*}
     */
    static from_client(raw_string) {
        return new EncString(raw_string ? encodeURI(raw_string) : '')
    }

    toJSON() {
        return this.__encoded
    }
}

export {EncString}