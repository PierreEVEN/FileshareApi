import {fetch_api} from "../../../utilities/request";
import {EncString} from "../../../types/encstring";
const {MODAL} = require("../../modal/modal");

/**
 * @param repository {number}
 * @param parent_item {number}
 */
function create_directory(repository, parent_item = null) {
    const widget = require('./create_directory.hbs')({}, {
        mkdir: async (e) => {
            e.preventDefault();
            console.log({
                name: EncString.from_client(document.getElementById('name').value),
                repository: repository,
                parent_item: parent_item
            })
            await fetch_api('item/new-directory/', 'POST',
                [{
                    name: EncString.from_client(document.getElementById('name').value),
                    repository: repository,
                    parent_item: parent_item
                }]
            );


            MODAL.close();
        }
    });
    MODAL.open(widget, {custom_width: '500px', custom_height: '250px'})
}

export {create_directory}