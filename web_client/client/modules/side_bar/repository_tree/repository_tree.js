import {EVENT_MANAGER} from "../../../types/event_manager";
import {context_menu_item} from "../../context_menu/contexts/context_item";
import {context_menu_repository} from "../../context_menu/contexts/context_repository";

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
        this._elements = div.elements;
        this._container.append(div);
    }

    /**
     * @param expanded {boolean}
     */
    async expand_node(expanded) {
        this._expanded = expanded;
        this._elements.content.innerHTML = '';
        if (expanded) {
            if (!this._listener)
                this._listener = EVENT_MANAGER.add('add_item', async (item) => {
                    if (item.parent_item === this._id) {
                        await new RepositoryNode(this._repository, this._elements.content, item.id).init();
                    }
                })
            const content = await this._repository.content.directory_content(this._id);
            for (const id of content) {
                const item = await this._repository.content.fetch_item(id);
                if (!item.is_regular_file) {
                    await new RepositoryNode(this._repository, this._elements.content, id).init();
                }
            }
            this._elements.category.classList.add('expand');
        } else {
            if (this._listener)
                this._listener.remove();
            delete this._listener;
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
        container.append(root_div);
    }

    /**
     * @param expanded {boolean}
     */
    async expand_node(expanded) {
        this._expanded = expanded;
        this._elements.content.innerHTML = '';
        if (expanded) {

            if (!this._listener)
                this._listener = EVENT_MANAGER.add('add_item', async (item) => {
                    if (!item.parent_item && item.repository === this.repository.id) {
                        await new RepositoryNode(this.repository, this._elements.content, item.id).init();
                    }
                })

            const content = await this.repository.content.root_content();
            for (const item_id of content) {
                await new RepositoryNode(this.repository, this._elements.content, item_id).init();
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
            if (this._listener)
                this._listener.remove();
            delete this._listener;
            this._elements.category.classList.remove('expand');
        }
    }
}

export {RepositoryTree}