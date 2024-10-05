import {ContextMenu, MenuAction} from "../context_menu";
import {create_directory} from "../../tools/create_directory/create_directory";
import {edit_repository} from "../../tools/edit_repository/edit_repository";

/**
 * @param repository {Repository}
 */
function context_menu_repository(repository) {
    const ctx = new ContextMenu();
    ctx.add_action(new MenuAction("Modifier", "/public/images/icons/icons8-edit-96.png", async () => {
        await edit_repository(repository);
    }, false));
    ctx.add_action(new MenuAction("Nouveau Dossier", "/public/images/icons/icons8-add-folder-48.png", async () => {
        create_directory(repository.id);
    }, false))
}

export {context_menu_repository}