import {ViewportContent} from "../../../types/viewport_content/viewport_content";
import {DirectoryContentProvider, RepositoryRootProvider} from "../../../types/viewport_content/providers";
import {ItemView} from "./content/item_view";
import {context_menu_repository} from "../../context_menu/contexts/context_repository";
import {Uploader} from "./upload/uploader";
import {DropBox} from "./upload/drop_box";

require('./repository_viewport.scss')

class RepositoryViewport {
    constructor(repository, container) {
        const div = require('./repository_viewport.hbs')({}, {
            background_context: (event) => {
                event.preventDefault();
                if (!event.target.classList.contains('file-list'))
                    return;
                context_menu_repository(repository);
            },
            open_upload: () => {
                this.open_upload_container()
                div.elements.upload_button.style.display = 'none';
            }
        });

        this.repository = repository;
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

        new DropBox(this._elements.drop_box, () => {
            if (!this.uploader)
                this.open_upload_container();
            return this.uploader;
        });
    }

    close_upload_container() {
        this._elements.upload_button.style.display = 'flex';
    }

    open_upload_container() {
        this._elements.upload_container.innerHTML = '';
        this.uploader = new Uploader(this._elements.upload_container, this)
        this.uploader.expand(true);
    }
}

export {RepositoryViewport}