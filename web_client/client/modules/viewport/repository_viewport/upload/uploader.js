import {UploadItem} from "./upload_item";
import {DirectoryContentProvider} from "../../../../types/viewport_content/providers";
import {UploadProcessor} from "./upload_processor";

require("./uploader.scss")

class Uploader {
    /**
     * @param container {HTMLElement}
     * @param viewport {RepositoryViewport}
     */
    constructor(container, viewport) {
        this.expanded = false;
        this.viewport = viewport;
        let div = require("./uploader.hbs")({}, {
            start_upload: () => this.set_uploading(!this.uploading),
            expand: (e) => {
                e.preventDefault();
                this.expand(!this.expanded)
            }
        });
        this._elements = div.elements;
        container.append(div);
        /**
         * @type {Map<string, UploadItem>}
         */
        this.children = new Map();

        this.uploading = false;

        this.handled_directories = new Map();
    }

    expand(expanded) {
        if (expanded === this.expanded)
            return;
        this.expanded = expanded;
        if (this.expanded)
            this._elements.uploader.classList.add("expanded")
        else
            this._elements.uploader.classList.remove("expanded")
    }

    set_uploading(uploading) {
        if (this.uploading === uploading)
            return;
        this.uploading = uploading;

        if (uploading) {
            this._elements.upload_button.style.display = 'none';
            this._elements.upload_in_progress.style.display = 'flex';
            this.start_upload();
        } else {
            this._elements.upload_in_progress.style.display = 'none';
            if (this.children.size > 0)
                this._elements.upload_button.style.display = 'flex';
        }
    }

    async start_upload() {

        /**
         * @param root {UploadItem}
         * @return {UploadItem}
         */
        const search_in_roots = (root) => {
            if (root.is_regular_file)
                return root;
            for (const ite of root.children.values()) {
                const found = search_in_roots(ite);
                if (found)
                    return found;
            }
            return null;
        }

        do {
            /**
             * @type {UploadItem}
             */
            let found_item = null;
            for (const root of this.children.values()) {
                const found = search_in_roots(root);
                if (found) {
                    found_item = found;
                    break;
                }
            }

            if (!found_item)
                break;

            this.processor = new UploadProcessor(found_item, this.viewport.repository.content);
            found_item.remove();
            await this.processor.upload();
        } while (true);
        this.viewport.close_upload_container();
    }

    /**
     * @param item {UploadItem}
     */
    async add_item(item) {

        const provider = this.viewport.content.get_content_provider();
        if (provider && provider.constructor.name === 'DirectoryContentProvider') {

            /**
             * @type {FilesystemItem}
             */
            const directory_item = provider['directory'];
            const directory = await this._add_existing_directory(directory_item);
            directory.add_child(item);
        } else {
            if (this.children.has(item.name))
                return;
            item.parent = this;
            this.children.set(item.name, item);
            item.instantiate(this._elements.file_list);
        }

        if (this.children.size > 0 && !this.uploading)
            this._elements.upload_button.style.display = 'flex';
    }

    /**
     * @param directory {FilesystemItem}
     * @return {Promise<UploadItem>}
     * @private
     */
    async _add_existing_directory(directory) {
        const existing = this.handled_directories.get(directory.id);
        if (existing) {
            return existing;
        } else {
            const entry = UploadItem.FromRepositoryDirectory(directory)
            if (directory.parent_item) {
                const parent = await this._add_existing_directory(await directory.filesystem().fetch_item(directory.parent_item));
                parent.add_child(entry);
            } else {
                this.children.set(entry.name, entry);
                entry.parent = this;
                entry.instantiate(this._elements.file_list);
            }

            this.handled_directories.set(directory.id, entry);
            return entry;
        }
    }

    _remove_child(name) {
        this.children.delete(name);
        if (this.children.size <= 0)
            this._elements.upload_button.style.display = 'none';
    }
}

export {Uploader}