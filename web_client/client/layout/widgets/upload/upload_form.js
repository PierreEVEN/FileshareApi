import {close_modal, is_modal_open, open_modal} from '../components/modal.js'
import {PAGE_CONTEXT, humanFileSize, seconds_to_str} from "../../../common/tools/utils.js";
import {print_message} from "../components/message_box.js";
import {Filesystem} from "../../../common/tools/filesystem.js";
import {FilesystemUpload} from "../../../common/tools/filesystem_upload.js";
import upload_hbs from "./upload_form.hbs";
import file_hbs from "./file.hbs";
import directory_hbs from "./directory.hbs";
import {spawn_context_action} from "../components/context_action";
import {FilesystemObject} from "../../../common/tools/filesystem_v2";
import {REPOS_BUILDER} from "../viewport/repos_builder"

require('./upload.scss')

const url = `${PAGE_CONTEXT.repos_path()}/send/`;
let filesystem = PAGE_CONTEXT.display_repos ? new Filesystem(PAGE_CONTEXT.display_repos.name) : null;
let stop_process = false;

let add_file_button = null;
let cancel_upload = null;
let upload_button = null;
let global_status_div = null;
let global_status_text = null;

function add_file_to_upload(file, path) {
    if (!is_modal_open()) open_upload_modal_for_files();
    filesystem.add_file(file, path ? path : '/');
}

let open_upload_modal_timeout = null;

function open_or_update_modal() {
    if (open_upload_modal_timeout) clearTimeout(open_upload_modal_timeout);
    open_upload_modal_timeout = setTimeout(() => {
        open_upload_modal_for_files();
        open_upload_modal_timeout = null;
    }, 100);
}

/**
 * @param viewport_filesystem {Filesystem}
 * @param search_dir {Directory}
 * @param viewport_id {number}
 */
function cleanup_path(viewport_filesystem, search_dir, viewport_id) {
    const server_objects = viewport_filesystem.get_objects_in_directory(viewport_id, null)

    const client_files = new Map()
    for (const object of search_dir.files)
        client_files.set(object.name, object);

    const client_dirs = new Map()
    for (const object of Object.values(search_dir.directories))
        client_dirs.set(object.name, object);

    for (const object of server_objects) {
        const viewport_obj_data = viewport_filesystem.get_object_data(object);
        if (!viewport_obj_data.is_regular_file)
            continue;
        const client_object = client_files.get(viewport_obj_data.name.plain());
        if (client_object)
            filesystem.remove_file(client_object);
    }
    for (const object of server_objects) {
        const viewport_obj_data = viewport_filesystem.get_object_data(object);
        if (viewport_obj_data.is_regular_file)
            continue;
        const local_dir = client_dirs.get(viewport_obj_data.name.plain());
        if (local_dir)
            cleanup_path(viewport_filesystem, local_dir, object);
    }
}

function cleanup_button() {
    const viewport_filesystem = REPOS_BUILDER.filesystem
    if (!viewport_filesystem)
        return;

    cleanup_path(viewport_filesystem, filesystem.root, REPOS_BUILDER.navigator.get_current_directory())
}

let filesystem_upload = null;
function open_upload_modal_for_files() {

    filesystem_upload = PAGE_CONTEXT.display_repos ? new FilesystemUpload(filesystem, url) : null;
    if (filesystem_upload) {
        filesystem_upload.callback_finished = () => {
            close_modal();
            print_message('info', 'Tache terminée', 'Mise en ligne des fichiers terminée avec succès.')
        }

        filesystem_upload.callback_file_uploaded = async (_, context) => {
            const file = await FilesystemObject.FetchFromServer(context.file_id);
            REPOS_BUILDER.filesystem.add_object(file);
        }
    }

    filesystem.clear();
    const gen_dir = (dir, parent_div) => {
        const ctx = {};
        const directory = directory_hbs({item: dir}, ctx);
        const dir_content = directory.getElementsByClassName('folder-content')[0];
        ctx.enter = () => directory.getElementsByClassName('cancel-button')[0].style.opacity = '1';
        ctx.leave = () => directory.getElementsByClassName('cancel-button')[0].style.opacity = '0';
        ctx.clicked = () => {
            if (!dir_content.expanded) {
                dir_content.expanded = true;
                if (!dir_content.generate_content) {
                    dir_content.generate_content = true;
                    for (const child_dir of Object.values(dir.directories)) gen_dir(child_dir, dir_content);

                    for (const file of dir.files) gen_file(file, dir_content);
                }
                dir_content.style.display = 'flex';
            } else {
                dir_content.expanded = false;
                dir_content.style.display = 'none';
            }
        }
        ctx.removed = () => dir.remove();

        const title = directory.getElementsByTagName('h2')[0];
        dir.callback_stats_updated = (content_size, content_files) => title.innerText = `${dir.name} (${humanFileSize(content_size)} - ${content_files} fichiers)`;
        dir.callback_stats_updated(dir.content_size, dir.content_files);
        dir.callback_directory_added = new_dir => {
            if (dir_content.generate_content) gen_dir(new_dir, dir_content);
        }
        dir.callback_file_added = new_file => {
            if (dir_content.generate_content) gen_file(new_file, dir_content);
        }

        dir.callback_removed = () => directory.remove();
        parent_div.append(directory);
    }

    const gen_file = (file, parent_div) => {
        const ctx = {
            removed: () => {
            }, enter: () => {
            }, leave: () => {
            }
        };
        const file_dom = file_hbs({item: file, name: file.name, size: humanFileSize(file.size)}, ctx);
        ctx.removed = () => filesystem.remove_file(file);
        ctx.enter = () => file_dom.getElementsByClassName('cancel-button')[0].style.opacity = '1';
        ctx.leave = () => file_dom.getElementsByClassName('cancel-button')[0].style.opacity = '0';
        file.callback_removed = () => file_dom.remove();
        parent_div.append(file_dom);
    }

    const modal_parent = open_modal(upload_hbs({}, {
        send: start_upload,
        pause: (button) => {
            if (button.paused) {
                button.paused = false;
                button.firstChild.src = '/images/icons/icons8-pause-30.png';
                filesystem_upload.start();
            } else {
                button.paused = true;
                button.firstChild.src = '/images/icons/icons8-play-64.png';
                filesystem_upload.pause();
            }
        }
    }), '80vw', '90vh', 'upload');

    const title = modal_parent.getElementsByTagName('h1')[0];
    const container = modal_parent.getElementsByClassName('file-list-box')[0];
    const global_progress_bar = modal_parent.getElementsByClassName('progress-bar')[0];
    const global_sub_progress_bar = modal_parent.getElementsByClassName('sub-progress-bar')[0];
    add_file_button = modal_parent.getElementsByClassName('plus-button')[0];
    upload_button = modal_parent.getElementsByClassName('confirm-button')[0];
    cancel_upload = modal_parent.getElementsByClassName('cancel-button')[0];
    global_status_div = modal_parent.getElementsByClassName('global-status')[0];
    global_status_text = global_status_div.getElementsByTagName('p')[0];
    filesystem.root.callback_stats_updated = (content_size, content_files) => title.innerText = content_files === 0 ? 'Envoyer des fichiers' : title.innerText = `${content_files} fichiers (${humanFileSize(content_size)})`;
    filesystem.root.callback_file_added = (new_file) => gen_file(new_file, container);
    filesystem.root.callback_directory_added = (new_dir) => gen_dir(new_dir, container);
    filesystem_upload.callback_update_progress = (file_name, file_size, file_sent, total_files, uploaded_bytes, file_uploaded_bytes, total_size, process_percent, speed, remaining) => {
        global_progress_bar.style.width = `${uploaded_bytes / total_size * 100}%`;
        global_sub_progress_bar.style.width = `${file_uploaded_bytes / total_size * 100}%`;
        global_status_text.innerText = `${Math.round(uploaded_bytes / total_size * 100)}% (${humanFileSize(uploaded_bytes)} / ${humanFileSize(total_size)}) - ${humanFileSize(speed)}/s (~${seconds_to_str(remaining)})\n${file_name} (${humanFileSize(file_size)})`;
        if (uploaded_bytes === file_uploaded_bytes && process_percent < 1.0)
            global_status_text.innerText += `\npost processing : ${Math.round(process_percent * 100)}%`
    }
    filesystem_upload.on_stop = (finished) => {
        add_file_button.style.display = "block";
        upload_button.style.display = "block";
        global_status_div.style.display = 'none';
        cancel_upload.value = "Annuler";
        cancel_upload.onclick = close_modal;
        stop_process = true;
        if (finished) {
            close_modal();
            print_message("Info", "Upload finished", "Successfully uploaded content")
        }
    }
    modal_parent.on_close_modal = () => {
        if (filesystem_upload.is_running) {
            filesystem_upload.pause();
            if (confirm('Un transfert est en cours, êtes vous sur de l\'interrompre ?')) {
                filesystem_upload.stop();
                filesystem.clear();
                return true;
            }
            filesystem_upload.start();
            return false;
        }
        return true;
    }
}

async function start_upload() {
    stop_process = false;
    add_file_button.style.display = "none";
    upload_button.style.display = "none";
    global_status_div.style.display = 'flex';
    cancel_upload.onclick = () => {
        filesystem_upload.stop()
    };
    cancel_upload.value = "Arrêter";
    const button = global_status_div.getElementsByTagName('button')[0];
    button.paused = false;
    button.firstChild.src = '/images/icons/icons8-pause-30.png';

    filesystem_upload.start();
}

function open_file_browser(directory) {
    const inputElement = document.createElement("input");
    inputElement.type = "file";
    if (directory) {
        inputElement.webkitdirectory = true;
        inputElement.directory = true;
        inputElement.multiple = true;
    } else {
        inputElement.multiple = true;
    }
    inputElement.addEventListener("change", (e) => {
        for (const file of e.target['files']) {
            const path = (file.webkitRelativePath ? file.webkitRelativePath : '').split('/');
            path.pop();
            add_file_to_upload(file, path.length > 0 ? path.join('/') : '')
        }
    })
    inputElement.dispatchEvent(new MouseEvent("click"));
}

function open_file_dialog() {
    spawn_context_action([{
        title: "Ajouter des fichiers",
        action: async () => open_file_browser(false),
        image: '/images/icons/icons8-file-96.png'
    }, {
        title: "Ajouter un dossier et son contenu",
        action: async () => open_file_browser(true),
        image: '/images/icons/icons8-folder-96.png'
    }])
}

window.upload = {add_file_to_upload, open_file_dialog, cleanup_button, open_or_update_modal}
export {add_file_to_upload, open_or_update_modal, open_file_dialog}