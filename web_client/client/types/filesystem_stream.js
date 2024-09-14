const {EncString} = require("./encstring");
const {User} = require("./user");
const {ContextMenu, MenuAction} = require("../modules/context_menu/context_menu");
const {fetch_api} = require("../utilities/request");
const {EVENT_MANAGER} = require("./event_manager");

/**
 * @type {Map<number, FilesystemStream>}
 * @private
 */
const _LOCAL_STORAGE = new Map();

class FilesystemItem {
    constructor(data) {
        /**
         * @type {number}
         */
        this.id = data.id;

        /**
         * @type {number}
         */
        this.repository = data.repository;

        /**
         * @type {number}
         */
        this.owner = data.owner;

        /**
         * @type {EncString}
         */
        this.name = new EncString(data.name);

        /**
         * @type {boolean}
         */
        this.is_regular_file = data.is_regular_file;

        /**
         * @type {EncString}
         */
        this.description = new EncString(data.description);

        /**
         * @type {number|null}
         */
        this.parent_item = data.parent_item;

        /**
         * @type {EncString}
         */
        this.absolute_path = new EncString(data.absolute_path);

        /**
         * @type {boolean}
         */
        this.in_trash = data.in_trash;

        /**
         * @type {null|Set<number>}
         */
        this.children = null;
    }

    static async new(data) {
        const item = new FilesystemItem(data);
        const filesystem = _LOCAL_STORAGE.get(item.repository)
        if (filesystem) {
            await filesystem.set_or_update_item(item);
        }
    }

    /**
     * @return {Repository}
     */
    display_data() {
        const result = JSON.parse(JSON.stringify(this));
        result.name = this.name.plain()
        result.description = this.description.plain()
        result.absolute_path = this.absolute_path.plain()
        return result
    }

    async refresh() {
        const storage = this.filesystem();
        if (storage)
            await storage.set_or_update_item(this);
    }

    /**
     * @return {FilesystemStream|null}
     */
    filesystem() {
        return _LOCAL_STORAGE.get(this.id);
    }

    async remove() {
        const fs = this.filesystem();
        await fs.remove_item(this);
    }
}

class FilesystemStream {

    /**
     * @param repository {Repository}
     */
    constructor(repository) {
        /**
         * @type {Repository}
         * @private
         */
        this._repository = repository;

        _LOCAL_STORAGE.set(this._repository.id, this);

        /**
         * @type {Promise<User>}
         * @private
         */
        this._user = new Promise(async (ok) => {
            ok(await User.find(this._repository.id));
        });

        /**
         * @type {Map<number, FilesystemItem>}
         * @private
         */
        this._items = new Map();

        /**
         * @type {Set<number>}
         * @private
         */
        this._roots = null;
    }

    /**
     * @param item_id {number}
     * @returns {Promise<FilesystemItem>}
     */
    async fetch_item(item_id) {
        const existing = this._items.get(item_id);
        if (existing) {
            return existing;
        }
        for (const item of await fetch_api(`item/find/`, 'POST', [item_id])) {
            await this.set_or_update_item(new FilesystemItem(item));
        }
        return this._items.get(item_id);
    }

    find(item_id) {
        return this._items.get(item_id);
    }

    /**
     * @param item_id {number}
     * @return {Promise<Set<number>>}
     */
    async directory_content(item_id) {
        const existing = await this.fetch_item(item_id);
        if (!existing)
            return new Set();
        if (existing.children) {
            return existing.children;
        }
        existing.children = new Set();
        for (const item of await fetch_api(`item/directory-content/`, 'POST', [item_id])) {
            await this.set_or_update_item(new FilesystemItem(item));
        }
        return existing.children;
    }

    /**
     * @return {Promise<Set<number>>}
     */
    async root_content() {
        if (!this._roots) {
            this._roots = new Set();
            for (const item of await fetch_api(`repository/root-content/`, 'POST', [this._repository.id])) {
                await this.set_or_update_item(new FilesystemItem(item));
            }
        }
        return this._roots
    }

    /**
     * @param item {FilesystemItem}
     */
    async set_or_update_item(item) {
        if (this._items.has(item.id)) {
            await this.remove_item(item);
        }
        this._items.set(item.id, item);
        if (item.parent_item !== undefined) {
            const parent = await this.fetch_item(item.parent_item);
            if (!parent.children)
                parent.children = new Set();
            parent.children.add(item.id);
        } else if (this._roots) {
            this._roots.add(item.id);
        }
        EVENT_MANAGER.broadcast('add_item', item);
    }

    /**
     * @param item {FilesystemItem}
     * @return {Promise<void>}
     */
    async remove_item(item) {
        if (item.parent_item) {
            const parent = await this.fetch_item(item.parent_item);
            parent.children.delete(item.id);
        } else {
            this._roots.delete(item.id);
        }
        EVENT_MANAGER.broadcast('remove_item', item);
    }
}


module.exports = {FilesystemStream, Item: FilesystemItem}