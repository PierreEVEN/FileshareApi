import {PAGE_CONTEXT, permissions} from "../common/tools/utils";
import {REPOS_BUILDER} from "./widgets/viewport/repos_builder"

const drop_box = document.createElement('div');
drop_box.classList.add('drop-box')

let WILL_DROP = null;

function reset_style() {
    drop_box.classList.remove('hover');
    drop_box.classList.remove('forbidden');
}

document.body.addEventListener('dragenter', async (event) => {
    if (!WILL_DROP) {
        WILL_DROP = new Promise(async (resolve) => {
            if (!PAGE_CONTEXT.display_repos)
                return resolve(false);
            const directory = REPOS_BUILDER.navigator.get_current_directory();
            if ((directory && await permissions.can_user_upload_to_directory(PAGE_CONTEXT.repos_path(), directory)) || await permissions.can_user_upload_to_repos(PAGE_CONTEXT.repos_path())) {
                resolve(true);
            } else
                resolve(false);
        })
    }

    event.preventDefault();

    if (!WILL_DROP || !await WILL_DROP) {
        drop_box.classList.add('forbidden');
        return;
    }

    drop_box.classList.add('hover');
})

document.body.addEventListener('mouseout', (event) => {
    WILL_DROP = null;
    reset_style();
    event.preventDefault();
})

document.body.addEventListener('dragover', async (event) => {
    event.preventDefault();
})

document.body.addEventListener('drop', async (event) => {
    reset_style();
    event.preventDefault();

    if (!WILL_DROP || !await WILL_DROP) {
        if (PAGE_CONTEXT.display_repos)
            window.location = `TODO OPEN UPLOAD PAGE HEHE`
        return;
    }

    if (!event.dataTransfer) {
        return;
    }

    if (event.dataTransfer.items) {
        [...event.dataTransfer.items].forEach((item, i) => {
            const process_entry = (entry, path) => {
                if (entry.isDirectory) {
                    path = `${path ? path : ''}/${entry.name}`;
                    entry.createReader().readEntries((entries) => {
                        for (const new_entry of entries)
                            process_entry(new_entry, path);
                    })
                } else {
                    entry.file(file => {
                        if (file.size === 0) return;
                        upload.add_file_to_upload(file, path);
                    });
                }
            }
            if (item.kind === "file") {
                const entry = "getAsEntry" in DataTransferItem.prototype ? item.getAsEntry() : item.webkitGetAsEntry();
                process_entry(entry);
            }
        });
    } else {
        [...event.dataTransfer.files].forEach((file, _) => {
            upload.add_file_to_upload(file, '/');
        });
    }
})


document.body.append(drop_box);
