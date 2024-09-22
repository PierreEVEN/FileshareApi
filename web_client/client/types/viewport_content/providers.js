import {ContentProvider} from "./viewport_content";

class RepositoryRootProvider extends ContentProvider {
    /**
     * @param repository {Repository}
     */
    constructor(repository) {
        super();
        this.repository = repository;
    }

    async get_content() {
        const items = [];
        for (const item_id of await this.repository.content.root_content()) {
            const item = await this.repository.content.fetch_item(item_id);
            if (!item.in_trash)
                items.push(item);
        }
        return items;
    }
}

class DirectoryContentProvider extends ContentProvider {
    /**
     * @param directory {FilesystemItem}
     */
    constructor(directory) {
        super();
        this.directory = directory;
    }

    async get_content() {
        const items = [];
        const fs = this.directory.filesystem();
        for (const item_id of await fs.directory_content(this.directory.id)) {
            const item = await fs.fetch_item(item_id);
            if (!item.in_trash)
                items.push(item);
        }
        return items;
    }
}

export {DirectoryContentProvider, RepositoryRootProvider}