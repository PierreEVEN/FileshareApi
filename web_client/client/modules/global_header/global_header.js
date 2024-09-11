require('./global_header.scss')

class GlobalHeader {
    /**
     * @param container {HTMLElement}
     */
    constructor(container) {
        /**
         * @type {null}
         * @private
         */
        this._connected_user = null;

        const div = require('./global_header.hbs')();
        this._elements = div['elements'];
        container.append(div);
    }

    set_connected_user(connected_user) {
        this._connected_user = connected_user;
    }
}

export {GlobalHeader}