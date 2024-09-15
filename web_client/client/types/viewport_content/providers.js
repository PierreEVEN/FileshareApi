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
        for (const item_id of await this.repository.content.root_content())
            items.push(await this.repository.content.fetch_item(item_id));
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
        for (const item_id of await fs.directory_content(this.directory.id))
            items.push(await fs.fetch_item(item_id));
        return items;
    }
}

export {DirectoryContentProvider, RepositoryRootProvider}