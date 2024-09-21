import {UploadItem} from "./upload_item";

require("./uploader.scss")

class Uploader {
    constructor() {
        this.expanded = false;
        let div = require("./uploader.hbs")({}, {
            start_upload: () => this.set_uploading(!this.uploading),
            expand: (e) => {
                e.preventDefault();
                this.expand(!this.expanded)
            }
        });
        this._elements = div.elements;
        document.body.append(div);
        this.children = new Map();

        this.uploading = false;
    }

    expand(expanded) {
        if (expanded === this.expanded)
            return;
        this.expanded = expanded;
        if (this.expanded)
            this._elements.uploader.classList.add("expanded")
        else
            this._elements.uploader.classList.remove("expanded")
    }

    set_uploading(uploading) {
        if (this.uploading === uploading)
            return;
        this.uploading = uploading;

        if (uploading) {
            this._elements.upload_button.style.display = 'none';
            this._elements.upload_in_progress.style.display = 'flex';
        } else {
            this._elements.upload_in_progress.style.display = 'none';
            if (this.children.size > 0)
                this._elements.upload_button.style.display = 'flex';
        }
    }

    /**
     * @param item {UploadItem}
     */
    add_item(item) {
        if (this.children.has(item.name))
            return;
        item.parent = this;
        this.children.set(item.name, item);
        item.instantiate(this._elements.file_list);

        if (this.children.size > 0 && !this.uploading)
            this._elements.upload_button.style.display = 'flex';
    }

    remove_child(name) {
        this.children.delete(name);
        if (this.children.size <= 0)
            this._elements.upload_button.style.display = 'none';
    }
}

let UPLOADER = null;

/**
 * @return {Uploader}
 */
function get_uploader() {
    if (!UPLOADER)
        UPLOADER = new Uploader();
    return UPLOADER;
}

get_uploader();

export {get_uploader}