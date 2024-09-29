import {fetch_api} from "../../../utilities/request";
import {EncString} from "../../../types/encstring";
import {MODAL} from "../../modal/modal";
import {Repository} from "../../../types/repository";

async function create_repository() {
    const widget = require('./create_repository.hbs')({}, {
        create_repository: async (e) => {
            e.preventDefault();
            const repositories = await fetch_api('repository/create/', 'POST',
                [{
                    name: EncString.from_client(document.getElementById('repository-name').value),
                    status: document.getElementById('repository-type').value
                }]
            );

            for (const repository of repositories) {
                Repository.new(repository);
            }

            MODAL.close();
        }
    });
    MODAL.open(widget, {custom_width: '500px', custom_height: '350px'})
}

export {create_repository}