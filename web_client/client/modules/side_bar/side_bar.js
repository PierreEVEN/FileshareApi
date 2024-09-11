import {fetch_api, fetch_user} from "../../utilities/request";
import {APP_CONFIG} from "../../types/app_config";

require('./side_bar.scss')

class SideBar {
    constructor(container) {
        /**
         * @type {User}
         * @private
         */
        this._connected_user = null;

        const div = require('./side_bar.hbs')({}, {
            switch_my_repos: () => {
                this.expand_my_repos(!this._my_repos_expanded);
            },
            switch_shared: () => {
                this.expand_shared(!this._shared_expanded);

            },
            switch_recent: () => {
                this.expand_recent(!this._recent_expanded);

            }
        });
        this._elements = div['elements'];
        container.append(div);

        document.addEventListener('on_connected_user_changed', async (new_user) => {
            console.log("new_user callback : ", new_user)
            await this.expand_my_repos(true)
        });
    }

    async expand_my_repos(expanded) {
        this._elements.shared.innerHTML = '';
        this._my_repos_expanded = expanded;
        console.log(await fetch_user('repositories/'))
    }

    async expand_shared(expanded) {
        this._elements.shared.innerHTML = '';
        this._shared_expanded = expanded;
        console.log(await fetch_user('repositories/shared/'))
    }

    expand_recent(expanded) {
        this._elements.recent.innerHTML = '';
        this._recent_expanded = expanded;
    }

    /**
     * @param connected_user {User}
     */
    refresh(connected_user) {
        this._connected_user = connected_user;
    }
}

export {SideBar}