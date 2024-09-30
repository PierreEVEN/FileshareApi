import './utilities/handlebars_helpers';
//@FIX : don't importing this cause a weird issue when rendering pdf...
require('./embed_viewers/custom_elements/pdf_viewer/pdf-viewer.hbs');
require('./embed_viewers/custom_elements/document/code');
require('./embed_viewers/custom_elements/document/markdown');
require('./embed_viewers/custom_elements/pdf_viewer/pdf-viewer');
require('./app.scss');

import {GlobalHeader} from "./modules/global_header/global_header";
import {SideBar} from "./modules/side_bar/side_bar";

import "./modules/viewport/repository_viewport/upload/uploader";
import {State} from "./utilities/state";
import {Repository} from "./types/repository";
import {Viewport} from "./modules/viewport/viewport";
import {APP_CONFIG} from "./types/app_config";
import {FilesystemItem} from "./types/filesystem_stream";

class FileshareApp {
    constructor() {

        /**
         * @type {HTMLElement}
         * @private
         */
        const layout = require('./app.hbs')({}, {});
        document.body.append(layout);

        /**
         * @type {object}
         * @private
         */
        this._elements = layout['elements'];

        /**
         * @type {Viewport}
         * @private
         */
        this._viewport = null;

        /**
         * @type {GlobalHeader}
         * @private
         */
        this._global_header = new GlobalHeader(this._elements.global_header);

        /**
         * @type {SideBar}
         * @private
         */
        this._side_bar = new SideBar(this, this._elements.side_bar);

        this.state = new State(this);

        (async () => {
            if (await APP_CONFIG.display_item()) {
                await this._side_bar.expand_to(APP_CONFIG.display_repository(), await APP_CONFIG.display_item(), false);
                await this.set_display_item(await APP_CONFIG.display_item());
            } else if (APP_CONFIG.display_repository()) {
                await this._side_bar.expand_to(APP_CONFIG.display_repository(), null, APP_CONFIG.in_trash());
                if (APP_CONFIG.in_trash())
                    await this.set_display_trash(APP_CONFIG.display_repository());
                else
                    await this.set_display_repository(APP_CONFIG.display_repository());
            } else if (APP_CONFIG.display_user()) {

            }
        })().catch(error => console.error(`initialization failed :`, error));
    }

    /**
     * @param repository {Repository}
     * @return {Promise<void>}
     */
    async set_display_repository(repository) {
        console.assert(repository, "invalid repository");
        const {Viewport} = require("./modules/viewport/viewport");
        if (!this._viewport)
            this._viewport = new Viewport(this._elements.viewport);
        await this.state.open_repository(repository);
        const viewport = await this._viewport.set_displayed_repository(repository);
        await viewport.open_root();
    }

    /**
     * @param item {FilesystemItem}
     * @return {Promise<void>}
     */
    async set_display_item(item) {
        const repository = await Repository.find(item.repository);
        if (!this._viewport)
            this._viewport = new Viewport(this._elements.viewport);
        const viewport = await this._viewport.set_displayed_repository(repository);
        await viewport.open_item(item);
        await this.state.open_item(item);
    }

    /**
     * @param repository {Repository}
     * @return {Promise<void>}
     */
    async set_display_trash(repository) {
        if (!this._viewport)
            this._viewport = new Viewport(this._elements.viewport);
        const viewport = await this._viewport.set_displayed_repository(repository);
        await viewport.open_trash();
        await this.state.open_trash(repository);
    }

    set_connected_user(user) {
        this._global_header.set_connected_user(user);

        if (user) {
            if (!this._side_bar) {
                this._side_bar = new SideBar(this, this._elements.side_bar);
            }
        }
        this._side_bar.refresh(user);
    }

}

const APP = new FileshareApp();

export {APP}