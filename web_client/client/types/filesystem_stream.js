const {EncString} = require("./encstring");
const {fetch_user, fetch_api} = require("../utilities/request");
const {Repository} = require("./repository");
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
}

class FilesystemStream {
    /**
     * @param repository_id {number}
     */
    constructor(repository_id) {
        /**
         * @type {Promise<Repository>}
         * @private
         */
        this._repository = Repository.find(repository_id);

        /**
         * @type {Promise<User>}
         * @private
         */
        this._user = new Promise(async (ok) => {
            ok(await User.find((await this._repository).id));
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
        for (const item of await fetch_api(`${(await this._user).name.encoded()}/${(await this._repository).url_name.encoded()}/item/`, 'POST', {items: [item_id]})) {
            await this._set_or_update_item(new Item(item));
        }
        return this._items.get(item_id);
    }

    /**
     * @param item_id {number}
     * @return {Promise<Map<number, Item>>}
     */
    async directory_content(item_id) {
        const existing = await this.fetch_item(item_id);
        if (!existing)
            return new Map();
        if (existing.children) {
            return existing.children;
        }
        existing.children = new Map();
        for (const item of await fetch_user(`${this._repository_name.encoded()}/directory-content/`, 'POST', {
            directories: [item_id]
        })) {
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
            for (const item of await fetch_user(`${this._repository_name.encoded()}/root-content/`, 'POST')) {
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
        if (item.parent_item) {
            const parent = await this.fetch_item(item.parent_item);
            if (!parent.children)
                parent.children = {};
            parent.children.add(item.id);
        } else if (this._roots) {
            this._roots.add(item.id);
        }
    }
}


module.exports = {FilesystemStream}