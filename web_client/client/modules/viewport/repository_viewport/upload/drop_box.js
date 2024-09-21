import {get_uploader} from "./uploader";
import {UploadItem} from "./upload_item";

class DropBox {
    constructor(box) {
        this.box = box;

        let WILL_DROP = null;

        document.body.addEventListener('dragenter', async (event) => {
            if (!WILL_DROP) {
                WILL_DROP = new Promise(async (resolve) => {
                    resolve(true);
                })
            }

            event.preventDefault();

            if (!WILL_DROP || !await WILL_DROP) {
                this.box.classList.add('forbidden');
                return;
            }

            this.box.classList.add('hover');
        })

        document.body.addEventListener('mouseout', (event) => {
            WILL_DROP = null;
            this.reset_style();
            event.preventDefault();
        })

        document.body.addEventListener('dragover', async (event) => {
            event.preventDefault();
        })

        document.body.addEventListener('drop', async (event) => {
            this.reset_style();
            event.preventDefault();

            if (!event.dataTransfer) {
                return;
            }

            if (event.dataTransfer.items) {
                const process_entry = async (entry, parent) => {

                    if (entry.isDirectory) {
                        const directory = await UploadItem.FromFilesystemDrop(entry);
                        if (parent)
                            parent.add_child(directory);
                        else
                            get_uploader().add_item(directory);
                        entry.createReader().readEntries((entries) => {
                            for (const new_entry of entries)
                                process_entry(new_entry, directory);
                        })
                    } else if (entry.isFile) {
                        if (parent)
                            parent.add_child(await UploadItem.FromFilesystemDrop(entry))
                        else
                            get_uploader().add_item(await UploadItem.FromFilesystemDrop(entry))
                    }
                }

                [...event.dataTransfer.items].forEach((item, i) => {
                    if (item.kind === "file") {
                        const entry = "getAsEntry" in DataTransferItem.prototype ? item.getAsEntry() : item.webkitGetAsEntry();
                        process_entry(entry, null);
                    }
                });
            } else {
                [...event.dataTransfer.files].forEach(async (file, _) => {
                    get_uploader().add_item(await UploadItem.FromFilesystemDrop(file))
                });
            }
        })

    }


    reset_style() {
        this.box.classList.remove('hover');
        this.box.classList.remove('forbidden');
    }

}

export {DropBox}