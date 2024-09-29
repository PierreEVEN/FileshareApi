import {RepositoryViewport} from "./repository_viewport/repository_viewport";
import {MemoryTracker} from "../../types/memory_handler";

class Viewport {
    /**
     * @param container {HTMLElement}
     */
    constructor(container) {
        /**
         * @type {HTMLElement}
         * @private
         */
        this._container = container;

        /**
         * @type {RepositoryViewport}
         * @private
         */
        this._viewport_object = null;
    }

    /**
     * @param repository {Repository}
     * @return {Promise<RepositoryViewport>}
     */
    async set_displayed_repository(repository) {
        if (!this._viewport_object || repository !== this._viewport_object.repository) {
            if (this._viewport_object)
            this.clear();
            this._viewport_object = new RepositoryViewport(repository, this._container);
        }
        return this._viewport_object;
    }

    clear() {
        this._container.innerHTML = '';
        if (this._viewport_object)
            this._viewport_object.delete();
        this._viewport_object = null;
    }
}

export {Viewport}