import {LOCAL_USER} from "../../../common/tools/user";
import {ClientString} from "../../../common/tools/client_string";
import {PAGE_CONTEXT, permissions} from "../../../common/tools/utils";
import {spawn_context_action} from "../components/context_action";
import {edit_repos} from "../edit_repos/edit_repos_form";
import {parse_fetch_result} from "../components/message_box";
import {REPOS_BUILDER} from "./repos_builder";

async function update_last_repos() {
    const repos_list = document.getElementById('last-repos-list')
    if (!repos_list)
        return;
    repos_list.innerHTML = '';
    const elements = await LOCAL_USER.get_last_repos();
    for (const repos of elements.reverse()) {
        const button = document.createElement('button');
        button.onclick = () => {
            window.location.href = `${window.origin}/${new ClientString(repos.username).encoded()}/${new ClientString(repos.name).encoded()}`;
        }
        button.classList.add('repos-list-item')
        if (PAGE_CONTEXT && PAGE_CONTEXT.display_repos && PAGE_CONTEXT.display_repos.id === repos.id)
            button.classList.add('selected-repos');

        const button_img = document.createElement('img');
        if (repos.status === 'public')
            button_img.src = '/images/icons/icons8-public-96.png';
        if (repos.status === 'hidden')
            button_img.src = '/images/icons/icons8-invisible-96.png';
        if (repos.status === 'private')
            button_img.src = '/images/icons/icons8-locked-96.png';
        const button_text = document.createElement('p');
        button_text.innerText = new ClientString(repos.name).plain();
        button.append(button_text);
        button.append(button_img);
        button.oncontextmenu = (event) => {
            open_repos_context_menu(repos.id);
            event.preventDefault();
        }
        button.repos_id = repos.id;
        repos_list.append(button)
    }
}

update_last_repos();

for (const button of document.getElementsByClassName('repos-list-item')) {
    button.oncontextmenu = (event) => {
        open_repos_context_menu(button.getAttribute('repos_id'));
        event.preventDefault();
    }
}

async function open_repos_context_menu(repos_id) {
    const actions = [];

    if (!REPOS_BUILDER)
        return;

    const repos_data = await parse_fetch_result(await fetch('/api/repos-data',
        {
            method: 'POST',
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json'
            },
            body: JSON.stringify([repos_id])
        }));
    if (repos_data.length > 0 && await permissions.can_user_edit_repos(`${window.origin}/${new ClientString(repos_data[0].username).for_url()}/${new ClientString(repos_data[0].name).for_url()}`)) {
        actions.push({
            title: "Informations & réglages",
            action: async () => {
                if (repos_data.length === 1) {
                    window.location.href = `${window.origin}/${new ClientString(repos_data[0].username).encoded()}/${new ClientString(repos_data[0].name).encoded()}/settings/`;
                }
            },
            image: '/images/icons/icons8-edit-96.png'
        });
    }
    if (Number(repos_id) === Number(PAGE_CONTEXT.display_repos.id)) {
        actions.push({
            title: "Corbeille",
            action: async () => {
                await REPOS_BUILDER.go_to_trash(true);
                await REPOS_BUILDER.path_builder.update_path();
            },
            image: '/images/icons/icons8-trash-96.png'
        });
    }
    spawn_context_action(actions);
}

window.file_list = {open_repos_context_menu}