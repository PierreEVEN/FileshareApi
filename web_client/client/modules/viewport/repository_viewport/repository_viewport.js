import {ViewportContent} from "../../../types/viewport_content/viewport_content";
import {
    DirectoryContentProvider,
    RepositoryRootProvider,
    TrashContentProvider
} from "../../../types/viewport_content/providers";
import {ItemView} from "./content/item_view";
import {context_menu_repository} from "../../context_menu/contexts/context_repository";
import {Uploader} from "./upload/uploader";
import {DropBox} from "./upload/drop_box";
import {MemoryTracker} from "../../../types/memory_handler";
import {context_menu_item} from "../../context_menu/contexts/context_item";
import {APP} from "../../../app";
import {ViewportToolbar} from "./toolbar/toolbar";

require('./repository_viewport.scss')

class RepositoryViewport extends MemoryTracker {
    constructor(repository, container) {
        super(RepositoryViewport);
        const div = require('./repository_viewport.hbs')({}, {
            background_context: (event) => {
                event.preventDefault();
                if (!event.target.classList.contains('file-list'))
                    return;
                if (this.content.get_content_provider() instanceof DirectoryContentProvider)
                    context_menu_item(this.content.get_content_provider().directory)
                else
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
                    await APP.set_display_item(item);
                }
            }))
        });

        this.toolbar = new ViewportToolbar(div.elements.toolbar, this.repository);

        this.content.events.add('remove', (item) => {
            const div = this._visible_items.get(item.id);
            if (div)
                div.remove();
        })

        container.append(div);

        new DropBox(this._elements.drop_box, () => {
            if (!this.uploader)
                this.open_upload_container();
            return this.uploader;
        });
    }

    /**
     * @param item {FilesystemItem}
     * @return {Promise<void>}
     */
    async open_item(item) {
        if (!item.is_regular_file)
           await this.content.set_content_provider(new DirectoryContentProvider(item));
        await this.toolbar.set_path_to(item, false);
    }


    async open_root() {
        if (this.content && (!this.content.get_content_provider() || !(this.content.get_content_provider() instanceof RepositoryRootProvider))) {
            await this.content.set_content_provider(new RepositoryRootProvider(this.repository));
            await this.toolbar.set_path_to(null, false);
        }
    }

    async open_trash() {
        if (this.content && (!this.content.get_content_provider() || !(this.content.get_content_provider() instanceof TrashContentProvider))) {
            await this.content.set_content_provider(new TrashContentProvider(this.repository));
            await this.toolbar.set_path_to(null, true);
        }
    }

    close_upload_container() {
        this._elements.upload_button.style.display = 'flex';
    }

    open_upload_container() {
        this._elements.upload_container.innerHTML = '';
        this.uploader = new Uploader(this._elements.upload_container, this)
        this.uploader.expand(true);
    }

    delete() {
        super.delete();
        if (this.content)
            this.content.delete();
    }
}

export {RepositoryViewport}