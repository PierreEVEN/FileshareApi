import {fetch_api} from "../../../../utilities/request";
import {FilesystemItem} from "../../../../types/filesystem_stream";
import {Message, NOTIFICATION} from "../message_box/notification";

/**
 * @param item {FilesystemItem}
 * @param move_to_trash {boolean}
 * @return {Promise<void>}
 */
async function delete_item(item, move_to_trash) {
    const fs = item.filesystem();
    const items = await fetch_api(`item/${move_to_trash ? 'move-to-trash' : 'delete'}/`, 'POST',
        [item.id]
    ).catch(error => NOTIFICATION.fatal(new Message(error).title("Impossible de supprimer le fichier")));
    if (move_to_trash)
        for (const item_id of items) {
            item.in_trash = true;
            await set_item_to_trash(await fs.fetch_item(item_id), true);
            await item.refresh();
        }
    else
        await item.remove();
}

/**
 * @param item {FilesystemItem}
 * @return {Promise<void>}
 */
async function restore_item(item) {
    const fs = item.filesystem();
    const items = await fetch_api(`item/restore/`, 'POST',
        [item.id]
    ).catch(error => NOTIFICATION.fatal(new Message(error).title("Impossible de restorer le fichier")));
    for (const item_id of items) {
        item.in_trash = false;
        await set_item_to_trash(await fs.fetch_item(item_id), false);
        await item.refresh();
    }
}

/**
 * @param item {FilesystemItem}
 * @param in_trash
 */
async function set_item_to_trash(item, in_trash) {
    item.in_trash = in_trash;
    if (item.children)
        for (const child of item.children)
            await set_item_to_trash(item.filesystem().find(child), in_trash);

    if (in_trash && (await item.filesystem().trash_content()).has(item.id)) {
        await item.refresh();
    }
}

export {delete_item, restore_item}