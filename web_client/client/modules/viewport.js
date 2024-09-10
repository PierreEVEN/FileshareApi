class Viewport {
    constructor() {
        /**
         * @type {Repository}
         * @private
         */
        this._displayed_repository = null;
    }

    set_displayed_repository(repository) {
        this.clear();
        this._displayed_repository = repository;
    }

    clear() {

    }
}

export {Viewport}