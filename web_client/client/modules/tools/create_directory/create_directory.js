import {fetch_api} from "../../../utilities/request";
import {EncString} from "../../../types/encstring";
import {FilesystemItem} from "../../../types/filesystem_stream";

const {MODAL} = require("../../modal/modal");

/**
 * @param repository {number}
 * @param parent_item {number}
 */
function create_directory(repository, parent_item = null) {
    const widget = require('./create_directory.hbs')({}, {
        mkdir: async (e) => {
            e.preventDefault();
            const directories = await fetch_api('item/new-directory/', 'POST',
                [{
                    name: EncString.from_client(document.getElementById('name').value),
                    repository: repository,
                    parent_item: parent_item
                }]
            );

            for (const item of directories) {
                await FilesystemItem.new(item);
            }

            MODAL.close();
        }
    });
    MODAL.open(widget, {custom_width: '500px', custom_height: '250px'})
}

export {create_directory}