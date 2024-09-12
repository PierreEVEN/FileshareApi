const {EncString} = require("./encstring");
const {fetch_repository} = require("../utilities/request");

class Item {
    constructor(data) {
        /**
         * @type {number}
         */
        this.id = data.id;
    }
}

class FilesystemStream {
    /**
     * @param repository_id {number}
     */
    constructor(repository_id) {
        /**
         * @type {number}
         * @private
         */
        this._repository_id = repository_id;

        /**
         * @type {Map<number, Item>}
         * @private
         */
        this._items = new Map();
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
        const item = await fetch_repository(`item/`, 'POST', [item_id]);
        this._items.set(item_id, new Item(item));
    }

    async directory_content(item_id) {
        const existing = await this.fetch_item(item_id);
        if (!existing)
            return [];
        if (existing.children) {
            return existing.children;
        }
        existing.children = new Map();
        for (const item of await fetch_repository(`content-of/${item_id}/`)) {
            existing.children.set(item.id, new Item(item))
        }
    }
}


module.exports = {FilesystemStream}