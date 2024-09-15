import {RepositoryViewport} from "./repository_viewport/repository_viewport";

class Viewport {
    /**
     * @param container {HTMLElement}
     */
    constructor(container) {
        /**
         * @type {Repository}
         * @private
         */
        this._displayed_repository = null;

        /**
         * @type {HTMLElement}
         * @private
         */
        this._container = container;
    }

    /**
     * @param repository {Repository}
     */
    set_displayed_repository(repository) {
        if (this._displayed_repository !== repository) {
            this.clear();
            this._displayed_repository = repository;
            if (repository) {
                new RepositoryViewport(repository, this._container);
            }
        }
    }

    clear() {

    }
}

export {Viewport}