import {GLOBAL_EVENTS} from "../../../types/event_manager";
import {context_menu_item} from "../../context_menu/contexts/context_item";
import {context_menu_repository} from "../../context_menu/contexts/context_repository";
import {APP} from "../../../app";

class RepositoryNode {
    /**
     * @param repository {Repository}
     * @param container {HTMLElement}
     * @param id {number}
     */
    constructor(repository, container, id) {
        this._repository = repository;
        this._id = id;
        this._expanded = false;
        this._container = container;
    }

    async init() {
        const data = await this._repository.content.fetch_item(this._id);
        const div = require('./repository_tree.hbs')(data.display_data(), {
            expand: async () => {
                await this.expand_node(!this._expanded);
            },
            context: (e) => {
                context_menu_item(data);
                e.preventDefault();
            }
        });
        this.data = data;
        this._div = div;
        this._elements = div.elements;
        this._container.append(div);
        return this;
    }

    /**
     * @param expanded {boolean}
     */
    async expand_node(expanded) {
        await APP.set_display_item(this.data);
        this._expanded = expanded;
        this._elements.content.innerHTML = '';
        if (expanded) {

            this._items = new Map();

            const add_item = async (item_id) => {
                if (!this._items.has(item_id))
                    this._items.set(item_id, await new RepositoryNode(this._repository, this._elements.content, item_id).init());
            }
            if (!this._listener_add)
                this._listener_add = GLOBAL_EVENTS.add('add_item', async (item) => {
                    if (item.parent_item === this._id && !item.in_trash && !item.is_regular_file) {
                        await add_item(item.id);
                    }
                })
            if (!this._listener_remove)
                this._listener_remove = GLOBAL_EVENTS.add('remove_item', async (item) => {
                    const item_node = this._items.get(item.id);
                    if (item_node) {
                        item_node._div.remove();
                        this._items.delete(item.id);
                    }
                })

            const content = await this._repository.content.directory_content(this._id);
            for (const id of content) {
                const item = await this._repository.content.fetch_item(id);
                if (!item.is_regular_file && !item.in_trash) {
                    await add_item(id);
                }
            }
            this._elements.category.classList.add('expand');
        } else {
            if (this._listener_add)
                this._listener_add.remove();
            delete this._listener_add;
            if (this._listener_remove)
                this._listener_remove.remove();
            delete this._listener_remove;
            this._elements.category.classList.remove('expand');
        }
    }
}

class RepositoryTree {

    /**
     * @param container {HTMLElement}
     * @param repository {Repository}
     */
    constructor(container, repository) {
        this.repository = repository;
        this._expanded = false;

        const root_div = require('./repository_tree_root.hbs')(repository.display_data(), {
            expand: async () => {
                await this.expand_node(!this._expanded)
            },
            context: (e) => {
                context_menu_repository(repository);
                e.preventDefault();
            }
        });
        this._elements = root_div.elements;
        this.root = root_div;
        container.append(root_div);
    }

    /**
     * @param expanded {boolean}
     */
    async expand_node(expanded) {
        await APP.set_display_repository(this.repository);
        this._expanded = expanded;
        this._elements.content.innerHTML = '';
        if (expanded) {

            this._items = new Map();

            const add_item = async (item_id) => {
                if (!this._items.has(item_id))
                    this._items.set(item_id, await new RepositoryNode(this.repository, this._elements.content, item_id).init());
            }

            if (!this._listener_add) {
                this._listener_add = GLOBAL_EVENTS.add('add_item', async (item) => {
                    if (item.parent_item === undefined && item.repository === this.repository.id && !item.in_trash && !item.is_regular_file) {
                        await add_item(item.id);
                    }
                })
            }
            if (!this._listener_remove)
                this._listener_remove = GLOBAL_EVENTS.add('remove_item', async (item) => {
                    const item_node = this._items.get(item.id);
                    if (item_node) {
                        item_node._div.remove();
                        this._items.delete(item.id);
                    }
                })

            const content = await this.repository.content.root_content();
            for (const item_id of content) {
                const item = await this.repository.content.fetch_item(item_id);
                if (!item.is_regular_file && !item.in_trash)
                    await add_item(item.id);
            }

            const trash_div = document.createElement('button');
            const trash_img = document.createElement('img');
            trash_img.src = "/public/images/icons/icons8-full-trash-96.png";
            trash_div.append(trash_img);
            const trash_txt = document.createElement('p');
            trash_txt.innerText = "corbeille";
            trash_div.append(trash_txt);
            this._elements.content.append(trash_div);
            this._elements.category.classList.add('expand');
        } else {
            if (this._listener_add)
                this._listener_add.remove();
            delete this._listener_add;
            if (this._listener_remove)
                this._listener_remove.remove();
            delete this._listener_remove;
            this._elements.category.classList.remove('expand');
        }
    }
}

export {RepositoryTree}