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

        /**
         * @type {MemoryTracker}
         * @private
         */
        this._viewport_object = null;
    }

    /**
     * @param repository {Repository}
     */
    set_displayed_repository(repository) {
        if (this._displayed_repository !== repository) {
            this.clear();
            this._displayed_repository = repository;
            if (repository) {
                if (this._viewport_object)
                    this._viewport_object.delete();
                this._viewport_object = new RepositoryViewport(repository, this._container);
            }
        }
    }

    clear() {
        this._container.innerHTML = '';
    }
}

export {Viewport}