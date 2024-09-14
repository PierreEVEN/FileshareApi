import {open_modal} from "../../../modules/modal/modal.js";

require('../../../modules/tools/create_repository/create-repos.scss')

const create_repos = require('../../../modules/tools/create_repository/create_repository.hbs')

function open_create_repos_modal() {
    open_modal(create_repos(), '500px', '350px', 'create-repos');
}

window.create_repos = {open_create_repos_modal}
export {open_create_repos_modal}
