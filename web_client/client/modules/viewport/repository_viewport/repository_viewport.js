import {ViewportContent} from "../../../types/viewport_content/viewport_content";
import {DirectoryContentProvider, RepositoryRootProvider} from "../../../types/viewport_content/providers";
import {ItemView} from "./content/item_view";
import {context_menu_repository} from "../../context_menu/contexts/context_repository";

require('./repository_viewport.scss')

class RepositoryViewport {
    constructor(repository, container) {

        const div = require('./repository_viewport.hbs')({}, {
            background_context: (event) => {
                event.preventDefault();
                context_menu_repository(repository);
            }
        });
        this._elements = div.elements;

        this.content = new ViewportContent();

        this._visible_items = new Map();

        this.content.events.add('add', (item) => {
            this._visible_items.set(item.id, new ItemView(item, this._elements.content, {
                clicked: async () => {
                    await this.content.set_content_provider(new DirectoryContentProvider(item));
                }
            }))
        })

        this.content.events.add('remove', (item) => {
            const div = this._visible_items.get(item.id);
            if (div)
                div.remove();
        })

        container.append(div);

        this.content.set_content_provider(new RepositoryRootProvider(repository));
    }
}

export {RepositoryViewport}