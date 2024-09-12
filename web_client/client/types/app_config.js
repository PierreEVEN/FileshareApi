import {User} from "./user";
import {Repository} from "./repository";
import {extend} from "dayjs";

class OnUserChanged extends Event {

}


class App_config {
    constructor() {
        const data = JSON.parse(document.body.dataset['app_config']);
        console.assert(data, "Invalid application configuration data")

        /**
         * @type {User}
         */
        this._connected_user = data.connected_user ? new User(data.connected_user) : null;
        /**
         * @type {User}
         */
        this._display_user = data.display_user ? new User(data.display_user) : null;
        /**
         * @type {Repository}
         */
        this._display_repository = data.display_repository ? new Repository(data.display_repository) : null;
        /**
         * @type {String}
         */
        this._origin = data.origin;
    }

    set_connected_user(new_user) {
        this._connected_user = new_user;
        document.dispatchEvent(new CustomEvent('on_connected_user_changed', {'detail': new_user}));
    }

    set_display_repository(display_user, display_repository) {
        this._display_user = display_user;
        this._display_repository = display_user ? display_repository : null;
        document.dispatchEvent(new CustomEvent('on_display_user_changed', {'detail': display_user}));
        document.dispatchEvent(new CustomEvent('on_display_repository_changed', {'detail': this._display_repository}));
    }

    set_display_user(new_user) {
        this.set_display_repository(new_user, null);
    }

    connected_user() {
        return this._connected_user;
    }

    display_user() {
        return this._display_user;
    }

    display_repository() {
        return this._display_repository;
    }

    origin() {
        return this._origin;
    }
}

/**
 * @type {App_config}
 */
const APP_CONFIG = new App_config();

export {APP_CONFIG}