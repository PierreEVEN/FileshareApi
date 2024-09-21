
import './layout/handlebars_helpers';
//@FIX : don't importing this cause a weird issue when rendering pdf...
require('./embed_viewers/custom_elements/pdf_viewer/pdf-viewer.hbs');
require('./embed_viewers/custom_elements/document/code');
require('./embed_viewers/custom_elements/document/markdown');
require('./embed_viewers/custom_elements/pdf_viewer/pdf-viewer');
require('./app.scss');

import {GlobalHeader} from "./modules/global_header/global_header";
import {SideBar} from "./modules/side_bar/side_bar";
import {GLOBAL_EVENTS} from "./types/event_manager";

import "./modules/viewport/repository_viewport/upload/uploader";
import {DropBox} from "./modules/viewport/repository_viewport/upload/drop_box";

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
        this._side_bar = new SideBar(this._elements.side_bar);

        const cb = GLOBAL_EVENTS.add('add_repository', (repository) => {
            this.set_display_repository(repository);
            cb.remove();
        })
    }

    set_display_repository(repository) {
        if (repository) {
            const {Viewport} = require("./modules/viewport/viewport");
            if (!this._viewport)
                this._viewport = new Viewport(this._elements.viewport);
            this._viewport.set_displayed_repository(repository);
        } else if (this._viewport) {
            this._viewport.clear();
            delete this._viewport;
        }
    }

    set_connected_user(user) {
        this._global_header.set_connected_user(user);

        if (user) {
            if (!this._side_bar) {
                this._side_bar = new SideBar(this._elements.side_bar);
            }
        }
        this._side_bar.refresh(user);
    }
}

const APP = new FileshareApp();

export {APP}