const mime = require('mime');

class UploadItem {
    constructor(data) {
        this.is_regular_file = data.is_regular_file;
        this.name = data.name;
        this.file = data.file;
        this.children = new Map();
        this.expanded = false;
        this.mimetype = data.mimetype;
        this.parent = null;
    }

    instantiate(container) {
        this.div = require("./item.hbs")({data: this, directory: !this.is_regular_file}, {
            expand: () => {
                this.set_expanded(!this.expanded)
            }
        });
        this._elements = this.div.elements;
        container.append(this.div)
    }

    set_expanded(expand) {
        this.expanded = expand;
        this._elements.content.innerHTML = '';
        if (expand) {
            for (const child of this.children.values())
                if (!child.is_regular_file)
                    child.instantiate(this._elements.content);
            for (const child of this.children.values())
                if (child.is_regular_file)
                    child.instantiate(this._elements.content);
            this.div.classList.add('expand');
        } else {
            this.div.classList.remove('expand');
        }
        for (const child of this.children.values()) {
            child.expanded = false;
        }
    }


    /**
     * @param fs_drop {FileSystemEntry}
     * @return {UploadItem}
     * @constructor
     */
    static async FromFilesystemDrop(fs_drop) {
        let file = null;
        if (fs_drop.isFile) {
            file = await new Promise((resolve) => {
                fs_drop.file(file => {
                    resolve(file);
                })
            })
            if (!file.type)
                file.mimetype = mime.getType(file.name);
            else
                file.mimetype = file.type;
        }
        return new UploadItem({
            is_regular_file: fs_drop.isFile,
            name: fs_drop.name,
            file: file,
            mimetype: file ? file.mimetype : null
        });
    }

    /**
     * @param item {UploadItem}
     */
    add_child(item) {
        if (this.children.has(item.name))
            return;
        this.children.set(item.name, item);
        item.parent = this;
    }

    remove_child(name) {
        this.children.delete(name);
    }

    remove() {
        this.div.remove();
        this.parent.remove_child(this.name);
    }
}

export {UploadItem}