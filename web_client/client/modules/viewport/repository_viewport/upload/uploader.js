import {UploadItem} from "./upload_item";
import {DirectoryContentProvider} from "../../../../types/viewport_content/providers";
import {UploadProcessor} from "./upload_processor";
import {EventManager} from "../../../../types/event_manager";
import {MemoryTracker} from "../../../../types/memory_handler";
import {humanFileSize} from "../../../../common/tools/utils";

require("./uploader.scss")

class Uploader extends MemoryTracker {
    /**
     * @param container {HTMLElement}
     * @param viewport {RepositoryViewport}
     */
    constructor(container, viewport) {
        super(Uploader);
        this.expanded = false;
        this.viewport = viewport;
        let div = require("./uploader.hbs")({}, {
            start_upload: () => this.set_uploading(!this.uploading),
            expand: (e) => {
                e.preventDefault();
                this.expand(!this.expanded)
            },
            pause: () => {
                this.set_pause(!this.pause);
            }
        });
        this.pause = false;
        this._elements = div.elements;
        this.total_items = 0;
        this.total_size = 0;
        container.append(div);
        /**
         * @type {Map<string, UploadItem>}
         */
        this.children = new Map();
        this.handled_directories = new Map();

        this.uploading = false;
        this.pause = false;
        this.event = new EventManager();
        this.total_to_upload = 0;
        this.uploaded = 0;
    }

    expand(expanded) {
        if (expanded === this.expanded)
            return;
        this.expanded = expanded;
        if (this.expanded)
            this._elements.uploader.classList.add("expanded")
        else {
            this._elements.uploader.classList.remove("expanded")
            if (this.children.size === 0)
                this.viewport.close_upload_container();
        }
    }

    set_pause(pause) {
        if (this.pause === pause)
            return;
        this._elements.pause_img.src = pause ? "/public/images/icons/icons8-play-64.png" : "/public/images/icons/icons8-pause-30.png";
        this.pause = pause;
        this.event.broadcast('pause', this.pause);
    }

    set_uploading(uploading) {
        if (this.uploading === uploading)
            return;
        this.uploading = uploading;

        if (uploading) {
            this._elements.upload_button.style.display = 'none';
            this._elements.upload_in_progress.style.display = 'flex';
            this.start_upload().catch(err => console.error("upload failed :", err));
        } else {
            this._elements.upload_in_progress.style.display = 'none';
            if (this.children.size > 0)
                this._elements.upload_button.style.display = 'flex';
        }
    }


    async start_upload() {

        this.total_to_upload = this.total_size;

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

            found_item.remove();

            if (found_item.parent instanceof UploadItem) {
                await found_item.parent.create_directory(this.viewport.repository.content);
            }
            const existing = await this.viewport.repository.content.find_child(found_item.name, found_item.parent.directory);
            if (existing) {
                console.warn("Existe deja !!")
                continue
            }

            if (this.processor)
                this.processor.delete()
            this.processor = new UploadProcessor(found_item, this);
            this._elements.upload_file_name.innerText = found_item.name;
            this._elements.upload_file_size.innerText = humanFileSize(found_item.file.size);
            await this.processor.upload().catch(() => {
            });
            this.uploaded += found_item.file.size;
        } while (true);
        this.viewport.close_upload_container();
    }

    /**
     * @param uploaded {number}
     * @param total {number}
     */
    progress(uploaded, total) {
        const current = (this.uploaded + uploaded) / this.total_to_upload;
        const after = (this.uploaded + total) / this.total_to_upload;
        this._elements.progress.style.width = `${current * 100}%`;
        this._elements.progress_after.style.width = `${after * 100}%`;
        this._elements.percents.innerText = `${Math.trunc(current * 100)}%`
    }

    /**
     * @param item {UploadItem}
     */
    async add_item(item) {
        const provider = this.viewport.content.get_content_provider();
        if (provider && provider instanceof DirectoryContentProvider) {
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
            this.parent_add_stats(item.total_size, item.total_items);
        }

        if (this.children.size > 0 && !this.uploading)
            this._elements.upload_button.style.display = 'flex';
    }

    parent_add_stats(size, count) {
        this.total_items += count;
        this.total_size += size;
        this._elements.infos.innerText = `${this.total_items} fichiers (${humanFileSize(this.total_size)})`;
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