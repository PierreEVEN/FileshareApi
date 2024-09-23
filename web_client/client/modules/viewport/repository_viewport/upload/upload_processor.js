import {print_message} from "../../../../layout/widgets/components/message_box";
import {FilesystemItem} from "../../../../types/filesystem_stream";

class UploadState {
    constructor(data) {
        this.id = data.id;
        this.finished = data.finished;
        if (data.item)
            this.item = FilesystemItem.new(data.item);
    }
}

class UploadProcessor {

    /**
     * @param item {UploadItem}
     * @param repository {Repository}
     */
    constructor(item, repository) {
        /**
         * @type {UploadItem}
         */
        this.item = item;
        /**
         * @type {Repository}
         */
        this.repository = repository;

        this.upload_signal = new Promise((resolve, reject) => {
            this.upload_finished = resolve;
            this.upload_failed = reject;
        })

        this._cursor = 0;
    }

    _init() {
        /**
         * @type {XMLHttpRequest}
         * @private
         */
        this._request = new XMLHttpRequest();

        this._request.onreadystatechange = () => {
            if (this._request.readyState === 4) {
                if (this._request.status === 0)
                    return this._fail("connection closed")
                if (this._request.response.length === 0)
                    return this._fail(`Unhandled response`, `${this._request.status}`)
                if (this._request.status !== 200)
                    return this._fail(this._request.response)
                this.state = new UploadState(JSON.parse(this._request.response));
                if (this.state.finished) {
                    return this.upload_finished();
                }
                else {
                    this._send_next();
                }
            }
        };

        this._request.upload.addEventListener("progress", (event) => {
        });

        this._send_next();
    }

    _fail(message) {
        print_message("error", `Error`, message);
        this.upload_failed();
    }


    static MAX_BATCH_SIZE = this.max_batch_size = 50 * 1024 * 1024;
    _send_next() {

        const start = this._cursor;
        this._cursor = Math.min(this._cursor + UploadProcessor.MAX_BATCH_SIZE, this.item.file.size)

        const chunk =  this.item.file.slice(start, this._cursor);

        this._request.open("POST", '/api/item/send/');
        if (!this.state) {
            this._request.setRequestHeader('Content-Name', encodeURIComponent(this.item.name));
            this._request.setRequestHeader('Content-Size', this.item.file.size.toString());
            this._request.setRequestHeader('Content-Timestamp', this.item.file.lastModified.toString());
            this._request.setRequestHeader('Content-Mimetype', encodeURIComponent(this.item.mimetype));
            this._request.setRequestHeader('Content-Repository', this.repository.id.toString());
            if (this.item.parent.directory)
                this._request.setRequestHeader('Content-Parent', this.item.parent.directory.id);
        } else {
            this._request.setRequestHeader('Content-Id', this.state.id);
        }
        this._request.send(chunk);
    }


    async upload() {
        if (this.item.parent.constructor.name === 'UploadItem')
            await this.item.parent.create_directory(this.repository.content);

        this._init();

        await this.upload_signal;
    }
}

export {UploadProcessor}