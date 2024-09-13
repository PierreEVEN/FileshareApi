const {EncString} = require("./encstring");
const {fetch_user, fetch_api} = require("../utilities/request");
const {User} = require("./user");

class Item {
    constructor(data) {
        /**
         * @type {number}
         */
        this.id = data.id;

        /**
         * @type {number}
         */
        this.repository_id = data.repository_id;

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

        /**
         * @type {Promise<User>}
         * @private
         */
        this._user = new Promise(async (ok) => {
            ok(await User.find(this._repository.id));
        });

        /**
         * @type {Map<number, Item>}
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
     * @returns {Promise<Item>}
     */
    async fetch_item(item_id) {
        const existing = this._items.get(item_id);
        if (existing) {
            return existing;
        }
        for (const item of await fetch_api(`item/find/`, 'POST', [item_id])) {
            await this._set_or_update_item(new Item(item));
        }
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
            await this._set_or_update_item(new Item(item));
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
                await this._set_or_update_item(new Item(item));
            }
        }
        return this._roots
    }

    /**
     * @param item {Item}
     * @private
     */
    async _set_or_update_item(item) {
        this._items.set(item.id, item);
        if (item.parent_item !== undefined) {
            const parent = await this.fetch_item(item.parent_item);
            if (!parent.children)
                parent.children = new Set();
            parent.children.add(item.id);
        } else if (this._roots) {
            this._roots.add(item.id);
        }
    }
}


module.exports = {FilesystemStream}