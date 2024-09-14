import {fetch_api, fetch_user} from "../../utilities/request";
import {APP_CONFIG} from "../../types/app_config";
import {Repository} from "../../types/repository";
import {APP} from "../../app";
import {APP_COOKIES} from "../../utilities/cookies";
import {User} from "../../types/user";
import {RepositoryTree} from "./repository_tree/repository_tree";
import {context_menu_my_repositories} from "../context_menu/contexts/context_my_repositories";

require('./side_bar.scss')

class SideBar {
    constructor(container) {
        /**
         * @type {User}
         * @private
         */
        this._connected_user = undefined;

        const div = require('./side_bar.hbs')({}, {
            expand_my_repositories: () => {
                this.expand_my_repositories(!this._my_repos_expanded);
            },
            switch_shared: () => {
                this.expand_shared(!this._shared_expanded);

            },
            switch_recent: () => {
                this.expand_recent(!this._recent_expanded);
            },
            context_my_repositories: (e) => {
                context_menu_my_repositories();
                e.preventDefault();
            }
        });
        this._elements = div['elements'];
        container.append(div);

        document.addEventListener('on_connected_user_changed', async (data) => {
            this.refresh(data.detail);
        });
        this.refresh(APP_CONFIG.connected_user());
    }

    async expand_my_repositories(expanded) {
        this._elements.my_repositories.innerHTML = '';
        this._my_repos_expanded = expanded;
        if (expanded) {
            this._elements.div_my_repositories.classList.add('expand');
            const my_repos = await fetch_api('repository/owned/');
            for (const repository of my_repos) {
                new RepositoryTree(this._elements.my_repositories, new Repository(repository));
            }
        }
        else {
            this._elements.div_my_repositories.classList.remove('expand');
        }
    }

    async expand_shared(expanded) {
        this._elements.shared.innerHTML = '';
        this._shared_expanded = expanded;
        if (expanded) {
            this._elements.div_shared.classList.add('expand');
            const my_repos = await fetch_api('repository/shared/');
            for (const repos of my_repos) {
                new RepositoryTree(this._elements.shared, new Repository(repos));
            }
        }
        else {
            this._elements.div_shared.classList.remove('expand');
        }
    }

    expand_recent(expanded) {
        this._elements.recent.innerHTML = '';
        this._recent_expanded = expanded;
    }

    /**
     * @param connected_user {User}
     */
    refresh(connected_user) {
        if (this._connected_user !== connected_user) {
            this._connected_user = connected_user;

            if (connected_user) {
                this._elements.div_my_repositories.style.display = 'flex';
                this._elements.div_shared.style.display = 'flex';
            }
            else {
                this._elements.div_my_repositories.style.display = 'none';
                this._elements.div_shared.style.display = 'none';
            }
        }
    }
}

export {SideBar}