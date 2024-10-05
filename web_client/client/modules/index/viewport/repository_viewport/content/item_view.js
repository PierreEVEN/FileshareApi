import {context_menu_item} from "../../../context_menu/contexts/context_item";

require('./item.scss')

class ItemView {
    /**
     * @param item {FilesystemItem}
     * @param container {HTMLElement}
     * @param events {object}
     */
    constructor(item, container, events = {}) {
        this.item = item;
        this.container = container;

        this.events = events;

        if (item.is_regular_file)
            this.div = require('./file.hbs')({item: item.display_data()}, {

                enter: () => {

                },
                leave: () => {

                },
                clicked: () => {
                    if (!item.in_trash)
                        this.events.clicked();
                },
                context_menu: (e) => {
                    context_menu_item(item);
                    e.preventDefault();
                }
            });
        else
            this.div = require('./directory.hbs')({item: item.display_data()}, {

                enter: () => {

                },
                leave: () => {

                },
                clicked: () => {
                    if (!item.in_trash)
                        this.events.clicked();
                },
                context_menu: (e) => {
                    context_menu_item(item);
                    e.preventDefault();
                }
            });
        container.append(this.div);
    }

    remove() {
        this.div.remove();
    }
}

export {ItemView}