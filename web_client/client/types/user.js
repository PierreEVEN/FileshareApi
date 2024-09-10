class UserRole {
    constructor(data) {
        switch (data) {
            case "guest":
            case "vip":
            case "admin":
                /**
                 * @type{string}
                 * @private
                 */
                this._role = data;
                break;
        }
    }

    to_string() {
        return this._role;
    }
}


class User {

    /**
     * @type {Map<number, User>}
     * @private
     */
    static _LOCAL_CACHE = new Map();

    constructor(data) {
        /**
         * @type {number}
         */
        this.id = data.id;
        /**
         * @type {EncString}
         */
        this.email = data.email;
        /**
         * @type {EncString}
         */
        this.name = data.name;
        /**
         * @type {EncString}
         */
        this.login = data.login;
        /**
         * @type {bool}
         */
        this.allow_contact = data.allow_contact;
        /**
         * @type {UserRole}
         */
        this.user_role = new UserRole(data.user_role);

        console.assert(!data['password_hash'])

        User._LOCAL_CACHE.set(this.id, this);
    }

    /**
     * @param id {number}
     * @returns {User}
     */
    find(id) {
        return User._LOCAL_CACHE.get(id);
    }
}

export {User, UserRole}