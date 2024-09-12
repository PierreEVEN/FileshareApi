
class RepositoryNode {
    /**
     * @param repository {Repository}
     * @param container {HTMLElement}
     * @param id {number}
     */
    constructor(repository, container, id) {
        this._repository = repository;
        this._container = container;
        this._id = id;

        container.append()
    }

    /**
     * @param node_id {number}
     * @param expanded {boolean}
     */
    expand_node(node_id, expanded) {
        this._repository.content
    }
}

class RepositoryTree {

    /**
     * @param container {HTMLElement}
     * @param repository {Repository}
     */
    constructor(container, repository) {
        this.repository = repository;

        /**
         * @type {Map<number, HTMLElement>}
         */
        this.nodes = new Map();


        container.append()
    }

    /**
     * @param node_id {number}
     * @param expanded {boolean}
     */
    expand_node(node_id, expanded) {

    }
}

export {RepositoryTree}