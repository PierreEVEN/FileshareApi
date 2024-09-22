import {EventManager} from "../event_manager";

class ContentFilter {
    constructor() {
        this._inner = null;
    }

    /**
     * @param item {FilesystemItem}
     * @returns {boolean}
     */
    test(item) {
        if (this._inner)
            if (!this._inner.test(item))
                return false;
        return this.filter(item);
    }

    /**
     * @param item {FilesystemItem}
     * @returns {boolean}
     */
    filter(item) {
        return true;
    }

    /**
     * @param filter {ContentFilter}
     * @return ContentFilter
     */
    join(filter) {
        this._inner = filter;
        return this._inner;
    }
}

class ContentProvider {
    constructor() {
    }

    /**
     * @return {Promise<FilesystemItem[]>}
     */
    async get_content() {
        return [];
    }
}

class ContentSorter {
    constructor() {
    }

    /**
     * @param source {FilesystemItem[]}
     * @return {FilesystemItem[]}
     */
    sort_content(source) {
        return source;
    }

    /**
     * @param sorter {ContentSorter}
     * @return ContentSorter
     */
    join(sorter) {
        this._inner = sorter;
        return this._inner;
    }
}

class ViewportContent {

    constructor() {

        /**
         * @type {Map<number, FilesystemItem>}
         * @private
         */
        this._displayed_items = new Map();

        /**
         * @type {EventManager}
         */
        this.events = new EventManager();

        /**
         * @type {ContentFilter|null}
         * @private
         */
        this._filter = null;

        /**
         * @type {ContentSorter|null}
         * @private
         */
        this._sorter = null;

        /**
         * @type {ContentProvider|null}
         * @private
         */
        this._provider = null;
    }

    /**
     * @param filter {ContentFilter|null}
     */
    async set_filter(filter) {
        this._filter = filter;
        await this._regen_content();
    }

    /**
     * @param sorter {ContentSorter|null}
     */
    async set_sorter(sorter) {
        this._sorter = sorter;
        await this._regen_content();
    }

    /**
     * @param provider {ContentProvider|null}
     */
    async set_content_provider(provider) {
        this._provider = provider;
        await this._regen_content();
    }

    /**
     * @return {ContentProvider|null}
     */
    get_content_provider() {
        return this._provider
    }

    _add(item) {
        if (this._displayed_items.has(item.id))
            return;
        this._displayed_items.set(item.id, item);
        this.events.broadcast('add', item);
    }

    _remove_entry(item) {
        if (!this._displayed_items.has(item.id))
            return;
        this._displayed_items.delete(item);
        this.events.broadcast('remove', item);
    }

    _clear() {
        for (const item of this._displayed_items.values())
            this._remove_entry(item);
    }

    async _regen_content() {
        this._clear();

        if (!this._provider)
            return;

        const sources = await this._provider.get_content();
        let filtered_sources = [];

        if (this._filter)
            for (const item of sources)
                filtered_sources.push(this._filter.test());
        else
            filtered_sources = sources;

        if (this._sorter)
            filtered_sources = this._sorter.sort_content(filtered_sources);

        for (const source of filtered_sources)
            this._add(source);
    }
}

export {ViewportContent, ContentFilter, ContentSorter, ContentProvider}