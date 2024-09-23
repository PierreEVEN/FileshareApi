import {EncString} from "./encstring";
import {FilesystemStream} from "./filesystem_stream";
import {fetch_api} from "../utilities/request";
import {GLOBAL_EVENTS} from "./event_manager";

class RepositoryStatus {
    constructor(data) {
        switch (data) {
            case "private":
            case "hidden":
            case "public":
                /**
                 * @type{string}
                 * @private
                 */
                this._role = data;
                break;
        }
    }

    toString() {
        return this._role;
    }
}


class Repository {

    /**
     * @type {Map<number, Repository>}
     * @private
     */
    static _LOCAL_CACHE = new Map();

    constructor(data) {
        /**
         * @type {number}
         */
        this.id = data.id;
        /**
         * @type {EncString}
         */
        this.url_name = new EncString(data.url_name);
        /**
         * @type {number}
         */
        this.owner = data.owner
        /**
         * @type {EncString}
         */
        this.description = new EncString(data.description);
        /**
         * @type {RepositoryStatus}
         */
        this.status = new RepositoryStatus(data.status);
        /**
         * @type {EncString}
         */
        this.display_name = new EncString(data.display_name);
        /**
         * @type {number}
         */
        this.max_file_size = data.max_file_size;
        /**
         * @type {number}
         */
        this.visitor_file_lifetime = data.visitor_file_lifetime;
        /**
         * @type {number}
         */
        this.allow_visitor_upload = data.allow_visitor_upload;

        /**
         * @type {FilesystemStream}
         */
        this.content = new FilesystemStream(this)

        if (Repository._LOCAL_CACHE.has(this.id))
            this.remove();

        Repository._LOCAL_CACHE.set(this.id, this);

        GLOBAL_EVENTS.broadcast('add_repository', this);
    }

    /**
     * @return {Repository}
     */
    display_data() {
        const result = JSON.parse(JSON.stringify(this));
        result.url_name = this.url_name.plain()
        result.description = this.description.plain()
        result.display_name = this.display_name.plain()
        return result
    }

    /**
     * @param id {number}
     * @returns {Promise<Repository>}
     */
    static async find(id) {
        const local = Repository._LOCAL_CACHE.get(id);
        if (local)
            return local;
        console.assert(id, "Invalid repository ID !");
        let repositories = await fetch_api('repository/find/', 'POST', [id]);
        for (const repository of repositories)
            new Repository(repository);

        return Repository._LOCAL_CACHE.get(id);
    }

    remove() {
        Repository._LOCAL_CACHE.delete(this.id);
        GLOBAL_EVENTS.broadcast('remove_repository', this);
    }

    toJSON() {
        const data = {};
        for (const [key, value] of Object.entries(this)) {
            if (key !== 'content')
                data[key] = value;
        }
        return data;
    }
}

export {Repository, RepositoryStatus}