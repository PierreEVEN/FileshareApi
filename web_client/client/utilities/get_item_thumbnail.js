import {get_mime_icon_path} from "./mime_utils";

function from_distant_repos(item) {
    const thumbnail_url = `/api/item/thumbnail/${item.id}/`;
    const mime = item.mimetype.split('/');
    switch (mime[0]) {
        case 'video':
            return `<div class="item-small">
                            <img class="item-background" src="${thumbnail_url}" alt="fichier: '${item.name}" onError="this.onError = undefined; src='/public/images/icons/mime-icons/video.png'"/>
                            <img class="item-overlay" src="/public/images/icons/icons8-play-64.png" alt="play button">
                        </div>`
        case 'image':
            return `<img class="item-small" src="${thumbnail_url}" alt="fichier: ${item.name}" onError="this.onError = null; this.src='/public/images/icons/mime-icons/image.png'"/>`
        case 'application':
            switch (mime[1]) {
                case 'x-pdf':
                case 'pdf':
                    return `<img class="item-small" src="${thumbnail_url}" alt="fichier: ${item.name}" onError="this.onError = null; this.src='/public/images/icons/mime-icons/image.png'"/>`
            }
            break;
    }

    return `<img class="item-small" src="${get_mime_icon_path(item.mimetype)}" alt="document: ${item.name}"/>`;
}

function from_local_path(item) {
    switch (item.mimetype.split('/')[0]) {
        case 'image':
            return `<img class="item-small" src="${URL.createObjectURL(item.file)}" alt="image: ${item.name}" onError="this.onError = null; this.src='/public/images/icons/mime-icons/image.png'"/>`
        case 'video':
            return `<video class="item-small" preload="auto" data-setup="{}" preload="auto" height="100%" width="100%">
                        <source src="${URL.createObjectURL(item.file)}" type="${item.mimetype}">
                    </video>`
    }
    return `<img class="item-small" src="${get_mime_icon_path(item.mimetype)}" alt="document: ${item.name}"/>`;
}


export {from_distant_repos, from_local_path}