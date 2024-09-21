import {MODAL} from "../modal/modal";
import {Authentication} from "../tools/authentication/authentication";
import {APP_CONFIG} from "../../types/app_config";
import {SIDE_BAR} from "../side_bar/side_bar";

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
        this._connected_user = undefined;

        const div = require('./global_header.hbs')({}, {
            login: () => {
                Authentication.login();
            },
            signup: () => {
                Authentication.signup();
            },
            logout: () => {
                Authentication.logout();
            },
            menu: () => {
                SIDE_BAR.show_mobile()
            }
        });


        document.addEventListener('on_connected_user_changed', async (data) => {
            this.refresh(data.detail);
        });

        this._elements = div['elements'];
        container.append(div);

        this.refresh(APP_CONFIG.connected_user())
    }

    refresh(connected_user) {
        if (connected_user !== this._connected_user) {
            this._connected_user = connected_user;
            if (connected_user) {
                this._elements.user.style.display = "flex";
                this._elements.signin.style.display = "none";
            }
            else {
                this._elements.user.style.display = "none";
                this._elements.signin.style.display = "flex";}
        }
    }
}

export {GlobalHeader}