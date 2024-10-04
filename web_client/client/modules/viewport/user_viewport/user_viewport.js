import {MemoryTracker} from "../../../types/memory_handler";
import {fetch_api} from "../../../utilities/request";
import {Message, NOTIFICATION} from "../../tools/message_box/notification";
import {Repository} from "../../../types/repository";
import {User} from "../../../types/user";
import {APP} from "../../../app";

require('./user_settings.scss')

class UserViewport extends MemoryTracker {
    /**
     * @param user {User}
     * @param container {HTMLElement}
     */
    constructor(user, container) {
        super(UserViewport);
        this.user = user;
        this.container = container;
        let viewport = require('./user_viewport.hbs')({user: user.display_data()}, {});
        container.append(viewport);
        this._elements = viewport.elements;
        this._fill_data();
    }

    async _fill_data() {
        let repositories = await fetch_api(`user/repositories/${this.user.id}/`)
            .catch(err => {
                NOTIFICATION.warn(new Message(err).title("Failed to retrieve user repositories"));
                return [];
            });

        for (const repository_id of repositories) {
            let repository = await Repository.find(repository_id);
            let widget = require('./repository.hbs')({text: repository.display_name.plain()}, {
                visit: async () => {
                    APP.set_display_repository(repository);
                }
            });
            this._elements.repository_list.append(widget);

        }
    }
}

export {UserViewport}