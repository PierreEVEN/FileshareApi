class RepositoryList {
    constructor() {
        /**
         * @type {User}
         * @private
         */
        this._connected_user = null;
    }

    /**
     * @param connected_user {User}
     */
    refresh(connected_user) {
        this._connected_user = connected_user;
    }
}

export {RepositoryList}