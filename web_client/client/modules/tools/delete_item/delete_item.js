import {fetch_api} from "../../../utilities/request";

/**
 * @param item {FilesystemItem}
 * @param move_to_trash {boolean}
 * @return {Promise<void>}
 */
async function delete_item(item, move_to_trash) {
    const items = await fetch_api(`item/${move_to_trash ? 'move-to-trash' : 'delete'}/`, 'POST',
        [item.id]
    );
    if (move_to_trash)
        for (const item_id of items) {
            set_item_to_trash(item_id, true);
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
    const items = await fetch_api(`item/restore/`, 'POST',
        [item.id]
    );
    for (const item_id of items)
        set_item_to_trash(item, false);
    await item.refresh();
}

/**
 * @param item {FilesystemItem}
 * @param in_trash
 */
function set_item_to_trash(item, in_trash) {
    item.in_trash = in_trash;
    for (const child of item.children)
        set_item_to_trash(item.filesystem.find(child), in_trash);
}

export {delete_item, restore_item}