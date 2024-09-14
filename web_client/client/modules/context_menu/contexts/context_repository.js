import {ContextMenu, MenuAction} from "../context_menu";
import {create_directory} from "../../tools/create_directory/create_directory";
import {delete_repository} from "../../tools/delete_repository/delete_repository";

/**
 * @param repository {Repository}
 */
function context_menu_repository(repository) {
    const ctx = new ContextMenu();
    ctx.add_action(new MenuAction("Nouveau Dossier", "public/images/icons/icons8-add-folder-48.png", async () => {
        create_directory(repository.id);
    }, false))
    ctx.add_action(new MenuAction("Supprimer", "public/images/icons/icons8-trash-96.png", async () => {
        await delete_repository(repository);
    }, false))
}

export {context_menu_repository}