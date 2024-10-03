import {EncString} from "./encstring";
import {fetch_api} from "../utilities/request";
import {Message, NOTIFICATION} from "../modules/tools/message_box/notification";

class UserRole {
    constructor(data) {
        switch (data) {
            case "Guest":
            case "Vip":
            case "Admin":
                /**
                 * @type{string}
                 * @private
                 */
                this._role = data;
                break;
            default:
                this._role = 'invalid'
                break;
        }
    }

    toString() {
        return this._role;
    }

    display_data() {
        switch (this._role) {
            case "Guest":
                return 'invité';
            case "Vip":
                return 'premium';
            case "Admin":
                return 'administrateur';
            default:
                return this._role;
        }
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

    /**
     * @return {User}
     */
    display_data() {
        const result = JSON.parse(JSON.stringify(this));
        result.name = this.name.plain()
        result.login = this.login.plain();
        result.email = this.email ? this.email.plain() : null;
        result.user_role = this.user_role.display_data();
        return result
    }
}

export {User, UserRole}