import {MemoryTracker} from "../../../types/memory_handler";
import {fetch_api} from "../../../utilities/request";
import {Message, NOTIFICATION} from "../../tools/message_box/notification";
import {Repository} from "../../../types/repository";

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
            console.log(repository)


        }
    }
}

export {UserViewport}