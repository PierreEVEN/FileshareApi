import {EncString} from "./encstring";
import {FilesystemStream} from "./filesystem_stream";
import {ContextMenu, MenuAction} from "../modules/context_menu/context_menu";
import {create_directory} from "../modules/tools/create_directory/create_directory";
import {fetch_api} from "../utilities/request";

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

        Repository._LOCAL_CACHE.set(this.id, this);
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

        let repositories = await fetch_api('repository/find/', 'POST', [id]);
        for (const repository of repositories)
            new Repository(repository);

        return Repository._LOCAL_CACHE.get(id);
    }

    toJSON() {
        const data = {};
        for (const [key, value] of Object.entries(this)) {
            if (key !== 'content')
                data[key] = value;
        }
        return data;
    }

    open_context_menu() {
        const ctx = new ContextMenu();
        ctx.add_action(new MenuAction("Nouveau Dossier", "public/images/icons/icons8-add-folder-48.png", async () => {
            create_directory(this.id);
        }, false))
        ctx.add_action(new MenuAction("Supprimer", "public/images/icons/icons8-trash-96.png", () => {
            this.delete_repository()
        }, false))
    }

    add_root_directory() {

    }

    delete_repository() {

    }
}

export {Repository, RepositoryStatus}