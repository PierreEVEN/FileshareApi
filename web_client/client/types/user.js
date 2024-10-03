import {EncString} from "./encstring";
import {fetch_api} from "../utilities/request";
import {Message, NOTIFICATION} from "../modules/tools/message_box/notification";

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
        this.email = new EncString(data.email);
        /**
         * @type {EncString}
         */
        this.name = new EncString(data.name);
        /**
         * @type {EncString}
         */
        this.login = new EncString(data.login);
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
     * @returns {Promise<User>}
     */
    static async find(id) {
        return User._LOCAL_CACHE.get(id);
    }

    /**
     * @param id {number}
     * @returns {Promise<User>}
     */
    static async fetch(id) {
        const current = User._LOCAL_CACHE.get(id);
        if (current)
            return current;
        let user = await fetch_api("user/find/", "POST", [id])
            .catch(error => NOTIFICATION.fatal(new Message(error).title(`Impossible de trouver l'utilisateur ${id}`)));
        if (user.length !== 0)
            return new User(user[0]);
        return null;
    }
}

export {User, UserRole}