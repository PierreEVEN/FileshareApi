import {fetch_api} from "../../utilities/request";
import {APP_CONFIG} from "../../types/app_config";
import {Repository} from "../../types/repository";
import {User} from "../../types/user";
import {RepositoryTree} from "./repository_tree/repository_tree";
import {context_menu_my_repositories} from "../context_menu/contexts/context_my_repositories";
import {GLOBAL_EVENTS} from "../../types/event_manager";

require('./side_bar.scss')

/**
 * @type {SideBar}
 */
let SIDE_BAR = null;

class SideBar {
    constructor(container) {

        SIDE_BAR = this;

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
        this.div = div;
        this._elements = div['elements'];
        container.append(div);

        document.addEventListener('on_connected_user_changed', async (data) => {
            this.refresh(data.detail);
        });
        this.refresh(APP_CONFIG.connected_user());

        this._my_repos_loaded = new Map();

        this._add_repository = GLOBAL_EVENTS.add('add_repository', async (repository) => {
            if (!this._my_repos_loaded.has(repository.id)) {
                this._my_repos_loaded.set(repository.id, new RepositoryTree(this._elements.my_repositories, repository));
            }
        });

        this._remove_repository = GLOBAL_EVENTS.add('remove_repository', async (repository) => {
            const my_repos_loaded = this._my_repos_loaded.get(repository.id);
            if (my_repos_loaded) {
                my_repos_loaded.root.remove();
                this._my_repos_loaded.delete(repository.id);
            }
        });

        this.expand_my_repositories(true);
    }

    async expand_my_repositories(expanded) {
        this._elements.my_repositories.innerHTML = '';
        this._my_repos_loaded = new Map()
        this._my_repos_expanded = expanded;
        if (expanded) {
            this._elements.div_my_repositories.classList.add('expand');
            const my_repos = await fetch_api('repository/owned/');
            for (const repository of my_repos) {
                new Repository(repository);
            }
        } else {
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
        } else {
            this._elements.div_shared.classList.remove('expand');
        }
    }

    expand_recent(expanded) {
        this._elements.recent.innerHTML = '';
        this._recent_expanded = expanded;
    }

    show_mobile() {
        console.log("ah")
        this.div.parentElement.style.display = 'flex';
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
            } else {
                this._elements.div_my_repositories.style.display = 'none';
                this._elements.div_shared.style.display = 'none';
            }
        }
    }

    remove() {
        if (this._add_repository)
            this._add_repository.remove();
        if (this._remove_repository)
            this._remove_repository.remove();
        delete this._remove_repository;
        delete this._add_repository;
    }
}

export {SideBar, SIDE_BAR}