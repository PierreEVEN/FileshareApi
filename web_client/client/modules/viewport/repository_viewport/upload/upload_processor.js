class UploadProcessor {

    /**
     * @param item {UploadItem}
     * @param filesystem {FilesystemStream}
     */
    constructor(item, filesystem) {
        this.item = item;
        this.filesystem = filesystem;
    }

    async upload() {
        if (this.item.parent.constructor.name === 'UploadItem')
            await this.item.parent.create_directory(this.filesystem);
        console.log("uploaded ", this.item.name)
    }
}

export {UploadProcessor}