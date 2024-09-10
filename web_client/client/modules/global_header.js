class GlobalHeader {
    constructor() {
        /**
         * @type {null}
         * @private
         */
        this._connected_user = null;
    }

    set_connected_user(connected_user) {
        this._connected_user = connected_user;
    }
}

export {GlobalHeader}