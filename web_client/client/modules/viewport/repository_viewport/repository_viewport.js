require('./repository_viewport.scss')

class RepositoryViewport {
    constructor(repository, container) {

        const div = require('./repository_viewport.hbs')({}, {

        });
        this._elements = div.elements;
        container.append(div);
    }
}

export {RepositoryViewport}