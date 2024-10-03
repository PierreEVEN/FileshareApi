import {humanFileSize} from "../../../../../common/tools/utils";
import {APP_CONFIG} from "../../../../../types/app_config";

class CarouselOverlay {
    /**
     * @param carousel {Carousel}
     * @param container {HTMLElement}
     * @param item
     */
    constructor(carousel, container, item) {
        this.carousel = carousel;
        this.container = container;

        this.carousel_overlay = require('./carousel_overlay.hbs')({
            item: item.display_data(),
            file_size: humanFileSize(item.size)
        }, {
            close_carousel: () => {
                carousel.close();
            },
            download: () => {
                window.open(`${APP_CONFIG.origin()}/api/item/${item.id}/`, '_blank').focus();
            },
            share: async () => {
                let url = `${APP_CONFIG.origin()}/api/item/${item.id}/`;
                await navigator.clipboard.writeText(url);
            }
        });
        container.firstChild.before(this.carousel_overlay)
    }

}

export {CarouselOverlay}