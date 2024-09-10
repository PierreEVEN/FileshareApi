class FileshareApp {
    constructor() {
        /**
         * @type {Viewport}
         * @private
         */
        this._viewport = null;

        /**
         * @type {GlobalHeader}
         * @private
         */
        this._global_header = null;

        /**
         * @type {RepositoryList}
         * @private
         */
        this._repository_list = null;

        /**
         * @type {HTMLElement}
         * @private
         */
        this._layout = require('./layout.hbs')({}, {});

        //this._layout.parse(["test"], this)
        console.log(this._layout, this._layout.parse);
        if (this._layout.parse)
            this._layout.parse(["test"], this);
        console.log(this._layout, this._layout.parse, this);

    }

    set_display_repository(user, repository) {
        if (repository) {
            const {Viewport} = require("./modules/viewport");
            if (!this._viewport)
                this._viewport = new Viewport();
            this._viewport.set_displayed_repository(repository);
        } else if (this._viewport) {
            this._viewport.clear();
            delete this._viewport;
        }
    }

    set_connected_user(user) {
        this._global_header.set_connected_user(user);

        if (user) {
            if (!this._repository_list) {
                const {RepositoryList} = require("./modules/repository_list");
                this._repository_list = new RepositoryList();
            }
        }
        this._repository_list.refresh(user);
    }
}

const APP = new FileshareApp();

export {APP}