(self["webpackChunkevenpierre"] = self["webpackChunkevenpierre"] || []).push([["viewers"],{

/***/ "./client/embed_viewers/custom_elements/lazy_image/lazy_image.js":
/*!***********************************************************************!*\
  !*** ./client/embed_viewers/custom_elements/lazy_image/lazy_image.js ***!
  \***********************************************************************/
/***/ (() => {

class LazyImage extends HTMLElement {
  constructor() {
    super();
    this.style.maxWidth = '100%';
    this.style.maxHeight = '100%';
    this.style.overflowX = 'hidden';
    this.style.overflowY = 'hidden';
    if (!this.hasAttribute('src')) return;
    if (this.hasAttribute('alternate-src')) {
      const tmp_image = new Image();
      tmp_image.classList.add('item-large');
      tmp_image.src = this.getAttribute('alternate-src');
      this.append(tmp_image);
    }
    //onError="this.onError = null; this.src='/images/icons/mime-icons/image.png'"
    const image = new Image();
    image.src = this.getAttribute('src');
    image.classList.add('item-large');
    image.onload = () => {
      this.innerHTML = '';
      this.append(image);
    };
  }
}
customElements.define("lazy-img", LazyImage);

/***/ }),

/***/ "./client/embed_viewers/distant_repos.js":
/*!***********************************************!*\
  !*** ./client/embed_viewers/distant_repos.js ***!
  \***********************************************/
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   get: () => (/* binding */ get)
/* harmony export */ });
/* harmony import */ var _common_tools_mime_utils__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ../common/tools/mime_utils */ "./client/common/tools/mime_utils.js");
/* harmony import */ var _common_tools_utils__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ../common/tools/utils */ "./client/common/tools/utils.js");


function get(item) {
  const url = `${_common_tools_utils__WEBPACK_IMPORTED_MODULE_1__.PAGE_CONTEXT.repos_path()}/file/${item.id}`;
  const thumbnail_url = `${_common_tools_utils__WEBPACK_IMPORTED_MODULE_1__.PAGE_CONTEXT.repos_path()}/thumbnail/${item.id}`;
  const mimetype = item.mimetype.plain().split('/');
  switch (mimetype[0]) {
    case 'image':
      return `<lazy-img class="item-large" src="${url}" alternate-src="${thumbnail_url}""/>`;
    case 'video':
      return `<video class="item-large video-js" preload="auto" data-setup="{}" autoplay="true" preload="auto" controls="true" height="100%" width="100%">
                        <source src="${url}" type="${item.mimetype}">
                    </video>`;
    case 'audio':
      return `<audio controls="true" src="${_common_tools_utils__WEBPACK_IMPORTED_MODULE_1__.PAGE_CONTEXT.repos_path()}/file/${item.id}"></audio>`;
    case 'application':
      switch (mimetype[1]) {
        case 'x-pdf':
        case 'pdf':
          return `<object data="${url}" type="application/pdf" width="100%" height="100%">
                                <pdf-embed src="${url}"></pdf-embed>
                            </object>`;
        case 'json':
        case 'x-json':
          return `<document-code src="${_common_tools_utils__WEBPACK_IMPORTED_MODULE_1__.PAGE_CONTEXT.repos_path()}/file/${item.id}" class="language-json"></document-code>`;
        case 'javascript':
        case 'x-javascript':
          return `<document-code src="${_common_tools_utils__WEBPACK_IMPORTED_MODULE_1__.PAGE_CONTEXT.repos_path()}/file/${item.id}" class="language-js"></document-code>`;
      }
      break;
    case 'text':
      switch (mimetype[1]) {
        case 'plain':
          if (item.name.plain().includes("log")) return `<document-code src="${_common_tools_utils__WEBPACK_IMPORTED_MODULE_1__.PAGE_CONTEXT.repos_path()}/file/${item.id}" class="language-log"></document-code>`;else return `<document-code src="${_common_tools_utils__WEBPACK_IMPORTED_MODULE_1__.PAGE_CONTEXT.repos_path()}/file/${item.id}" class="language-plain"></document-code>`;
        case 'markdown':
        case 'x-markdown':
          return `<document-markdown src="${_common_tools_utils__WEBPACK_IMPORTED_MODULE_1__.PAGE_CONTEXT.repos_path()}/file/${item.id}"></document-markdown>`;
        case 'scss':
        case 'x-scss':
          return `<document-code src="${_common_tools_utils__WEBPACK_IMPORTED_MODULE_1__.PAGE_CONTEXT.repos_path()}/file/${item.id}" class="language-scss"></document-code>`;
        case 'sass':
        case 'x-sass':
          return `<document-code src="${_common_tools_utils__WEBPACK_IMPORTED_MODULE_1__.PAGE_CONTEXT.repos_path()}/file/${item.id}" class="language-scss"></document-code>`;
        case 'css':
        case 'x-css':
          return `<document-code src="${_common_tools_utils__WEBPACK_IMPORTED_MODULE_1__.PAGE_CONTEXT.repos_path()}/file/${item.id}" class="language-css"></document-code>`;
      }
      return `<document-embed src="${_common_tools_utils__WEBPACK_IMPORTED_MODULE_1__.PAGE_CONTEXT.repos_path()}/file/${item.id}"></document-embed>`;
  }
  return `<img class="item-small" src="${(0,_common_tools_mime_utils__WEBPACK_IMPORTED_MODULE_0__.get_mime_icon_path)(item.mimetype)}" alt="document: ${item.name}"/>`;
}


/***/ }),

/***/ "./client/embed_viewers/index.js":
/*!***************************************!*\
  !*** ./client/embed_viewers/index.js ***!
  \***************************************/
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony import */ var handlebars__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! handlebars */ "./node_modules/handlebars/dist/cjs/handlebars.js");
/* harmony import */ var handlebars__WEBPACK_IMPORTED_MODULE_0___default = /*#__PURE__*/__webpack_require__.n(handlebars__WEBPACK_IMPORTED_MODULE_0__);
/* harmony import */ var _common_tools_mime_utils__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ../common/tools/mime_utils */ "./client/common/tools/mime_utils.js");


__webpack_require__(/*! ./custom_elements/document/code */ "./client/embed_viewers/custom_elements/document/code.js");
__webpack_require__(/*! ./custom_elements/document/markdown */ "./client/embed_viewers/custom_elements/document/markdown.js");
__webpack_require__(/*! ./custom_elements/lazy_image/lazy_image */ "./client/embed_viewers/custom_elements/lazy_image/lazy_image.js");
__webpack_require__(/*! ./custom_elements/pdf_viewer/pdf-viewer */ "./client/embed_viewers/custom_elements/pdf_viewer/pdf-viewer.js");
function mime_image_generator_helper_big(item) {
  // CASE : IS DIRECTORY
  if (!item.is_regular_file) {
    return new (handlebars__WEBPACK_IMPORTED_MODULE_0___default().SafeString)(`<img src="/images/icons/icons8-folder-96.png" alt="dossier: ${item.name}">`);
  }
  // CASE : IS STANDARD FILE
  else {
    if (!(0,_common_tools_mime_utils__WEBPACK_IMPORTED_MODULE_1__.is_mimetype_valid)(item.mimetype)) return new (handlebars__WEBPACK_IMPORTED_MODULE_0___default().SafeString)(`<img class="item-small" src="${(0,_common_tools_mime_utils__WEBPACK_IMPORTED_MODULE_1__.get_mime_icon_path)(item.mimetype)}" alt="document: ${item.name}"/>`);
    // Distant repos
    if (item.id) {
      return new (handlebars__WEBPACK_IMPORTED_MODULE_0___default().SafeString)((__webpack_require__(/*! ./distant_repos */ "./client/embed_viewers/distant_repos.js").get)(item));
    }
    // Filesystem file
    else if (item.lastModified) {
      return new (handlebars__WEBPACK_IMPORTED_MODULE_0___default().SafeString)(`<img class="item-small" src="${(0,_common_tools_mime_utils__WEBPACK_IMPORTED_MODULE_1__.get_mime_icon_path)(item.mimetype)}" alt="document: ${item.name}"/>`);
    }
  }
}
handlebars__WEBPACK_IMPORTED_MODULE_0___default().registerHelper("item_image", options => mime_image_generator_helper_big(options));
console.info('Loaded viewers.js');

/***/ })

},
/******/ __webpack_require__ => { // webpackRuntimeModules
/******/ var __webpack_exec__ = (moduleId) => (__webpack_require__(__webpack_require__.s = moduleId))
/******/ var __webpack_exports__ = (__webpack_exec__("./client/embed_viewers/index.js"));
/******/ }
]);
//# sourceMappingURL=data:application/json;charset=utf-8;base64,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bnQvZW1iZWRfdmlld2Vycy9jdXN0b21fZWxlbWVudHMvbGF6eV9pbWFnZS9sYXp5X2ltYWdlLmpzIiwid2VicGFjazovL2V2ZW5waWVycmUvLi9jbGllbnQvZW1iZWRfdmlld2Vycy9kaXN0YW50X3JlcG9zLmpzIiwid2VicGFjazovL2V2ZW5waWVycmUvLi9jbGllbnQvZW1iZWRfdmlld2Vycy9pbmRleC5qcyJdLCJzb3VyY2VzQ29udGVudCI6WyJjbGFzcyBMYXp5SW1hZ2UgZXh0ZW5kcyBIVE1MRWxlbWVudCB7XHJcbiAgICBjb25zdHJ1Y3RvcigpIHtcclxuICAgICAgICBzdXBlcigpO1xyXG5cclxuICAgICAgICB0aGlzLnN0eWxlLm1heFdpZHRoID0gJzEwMCUnO1xyXG4gICAgICAgIHRoaXMuc3R5bGUubWF4SGVpZ2h0ID0gJzEwMCUnO1xyXG4gICAgICAgIHRoaXMuc3R5bGUub3ZlcmZsb3dYID0gJ2hpZGRlbic7XHJcbiAgICAgICAgdGhpcy5zdHlsZS5vdmVyZmxvd1kgPSAnaGlkZGVuJztcclxuXHJcbiAgICAgICAgaWYgKCF0aGlzLmhhc0F0dHJpYnV0ZSgnc3JjJykpXHJcbiAgICAgICAgICAgIHJldHVybjtcclxuXHJcbiAgICAgICAgaWYgKHRoaXMuaGFzQXR0cmlidXRlKCdhbHRlcm5hdGUtc3JjJykpIHtcclxuICAgICAgICAgICAgY29uc3QgdG1wX2ltYWdlID0gbmV3IEltYWdlKCk7XHJcbiAgICAgICAgICAgIHRtcF9pbWFnZS5jbGFzc0xpc3QuYWRkKCdpdGVtLWxhcmdlJyk7XHJcbiAgICAgICAgICAgIHRtcF9pbWFnZS5zcmMgPSB0aGlzLmdldEF0dHJpYnV0ZSgnYWx0ZXJuYXRlLXNyYycpO1xyXG4gICAgICAgICAgICB0aGlzLmFwcGVuZCh0bXBfaW1hZ2UpO1xyXG4gICAgICAgIH1cclxuICAgICAgICAvL29uRXJyb3I9XCJ0aGlzLm9uRXJyb3IgPSBudWxsOyB0aGlzLnNyYz0nL2ltYWdlcy9pY29ucy9taW1lLWljb25zL2ltYWdlLnBuZydcIlxyXG4gICAgICAgIGNvbnN0IGltYWdlID0gbmV3IEltYWdlKCk7XHJcbiAgICAgICAgaW1hZ2Uuc3JjID0gdGhpcy5nZXRBdHRyaWJ1dGUoJ3NyYycpO1xyXG4gICAgICAgIGltYWdlLmNsYXNzTGlzdC5hZGQoJ2l0ZW0tbGFyZ2UnKTtcclxuICAgICAgICBpbWFnZS5vbmxvYWQgPSAoKSA9PiB7XHJcbiAgICAgICAgICAgIHRoaXMuaW5uZXJIVE1MID0gJyc7XHJcbiAgICAgICAgICAgIHRoaXMuYXBwZW5kKGltYWdlKTtcclxuICAgICAgICB9XHJcbiAgICB9XHJcblxyXG59XHJcblxyXG5jdXN0b21FbGVtZW50cy5kZWZpbmUoXCJsYXp5LWltZ1wiLCBMYXp5SW1hZ2UpOyIsIiAgICBpbXBvcnQge2dldF9taW1lX2ljb25fcGF0aH0gZnJvbSBcIi4uL2NvbW1vbi90b29scy9taW1lX3V0aWxzXCI7XHJcbmltcG9ydCB7UEFHRV9DT05URVhUfSBmcm9tIFwiLi4vY29tbW9uL3Rvb2xzL3V0aWxzXCI7XHJcblxyXG5mdW5jdGlvbiBnZXQoaXRlbSkge1xyXG4gICAgY29uc3QgdXJsID0gYCR7UEFHRV9DT05URVhULnJlcG9zX3BhdGgoKX0vZmlsZS8ke2l0ZW0uaWR9YDtcclxuICAgIGNvbnN0IHRodW1ibmFpbF91cmwgPSBgJHtQQUdFX0NPTlRFWFQucmVwb3NfcGF0aCgpfS90aHVtYm5haWwvJHtpdGVtLmlkfWA7XHJcbiAgICBjb25zdCBtaW1ldHlwZSA9IGl0ZW0ubWltZXR5cGUucGxhaW4oKS5zcGxpdCgnLycpO1xyXG4gICAgc3dpdGNoIChtaW1ldHlwZVswXSkge1xyXG4gICAgICAgIGNhc2UgJ2ltYWdlJzpcclxuICAgICAgICAgICAgcmV0dXJuIGA8bGF6eS1pbWcgY2xhc3M9XCJpdGVtLWxhcmdlXCIgc3JjPVwiJHt1cmx9XCIgYWx0ZXJuYXRlLXNyYz1cIiR7dGh1bWJuYWlsX3VybH1cIlwiLz5gXHJcbiAgICAgICAgY2FzZSAndmlkZW8nOlxyXG4gICAgICAgICAgICByZXR1cm4gYDx2aWRlbyBjbGFzcz1cIml0ZW0tbGFyZ2UgdmlkZW8tanNcIiBwcmVsb2FkPVwiYXV0b1wiIGRhdGEtc2V0dXA9XCJ7fVwiIGF1dG9wbGF5PVwidHJ1ZVwiIHByZWxvYWQ9XCJhdXRvXCIgY29udHJvbHM9XCJ0cnVlXCIgaGVpZ2h0PVwiMTAwJVwiIHdpZHRoPVwiMTAwJVwiPlxyXG4gICAgICAgICAgICAgICAgICAgICAgICA8c291cmNlIHNyYz1cIiR7dXJsfVwiIHR5cGU9XCIke2l0ZW0ubWltZXR5cGV9XCI+XHJcbiAgICAgICAgICAgICAgICAgICAgPC92aWRlbz5gXHJcbiAgICAgICAgY2FzZSAnYXVkaW8nOlxyXG4gICAgICAgICAgICByZXR1cm4gYDxhdWRpbyBjb250cm9scz1cInRydWVcIiBzcmM9XCIke1BBR0VfQ09OVEVYVC5yZXBvc19wYXRoKCl9L2ZpbGUvJHtpdGVtLmlkfVwiPjwvYXVkaW8+YFxyXG4gICAgICAgIGNhc2UgJ2FwcGxpY2F0aW9uJzpcclxuICAgICAgICAgICAgc3dpdGNoIChtaW1ldHlwZVsxXSkge1xyXG4gICAgICAgICAgICAgICAgY2FzZSAneC1wZGYnOlxyXG4gICAgICAgICAgICAgICAgY2FzZSAncGRmJzpcclxuICAgICAgICAgICAgICAgICAgICByZXR1cm4gYDxvYmplY3QgZGF0YT1cIiR7dXJsfVwiIHR5cGU9XCJhcHBsaWNhdGlvbi9wZGZcIiB3aWR0aD1cIjEwMCVcIiBoZWlnaHQ9XCIxMDAlXCI+XHJcbiAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgPHBkZi1lbWJlZCBzcmM9XCIke3VybH1cIj48L3BkZi1lbWJlZD5cclxuICAgICAgICAgICAgICAgICAgICAgICAgICAgIDwvb2JqZWN0PmBcclxuICAgICAgICAgICAgICAgIGNhc2UgJ2pzb24nOlxyXG4gICAgICAgICAgICAgICAgY2FzZSAneC1qc29uJzpcclxuICAgICAgICAgICAgICAgICAgICByZXR1cm4gYDxkb2N1bWVudC1jb2RlIHNyYz1cIiR7UEFHRV9DT05URVhULnJlcG9zX3BhdGgoKX0vZmlsZS8ke2l0ZW0uaWR9XCIgY2xhc3M9XCJsYW5ndWFnZS1qc29uXCI+PC9kb2N1bWVudC1jb2RlPmBcclxuICAgICAgICAgICAgICAgIGNhc2UgJ2phdmFzY3JpcHQnOlxyXG4gICAgICAgICAgICAgICAgY2FzZSAneC1qYXZhc2NyaXB0JzpcclxuICAgICAgICAgICAgICAgICAgICByZXR1cm4gYDxkb2N1bWVudC1jb2RlIHNyYz1cIiR7UEFHRV9DT05URVhULnJlcG9zX3BhdGgoKX0vZmlsZS8ke2l0ZW0uaWR9XCIgY2xhc3M9XCJsYW5ndWFnZS1qc1wiPjwvZG9jdW1lbnQtY29kZT5gXHJcbiAgICAgICAgICAgIH1cclxuICAgICAgICAgICAgYnJlYWs7XHJcbiAgICAgICAgY2FzZSAndGV4dCc6XHJcbiAgICAgICAgICAgIHN3aXRjaCAobWltZXR5cGVbMV0pIHtcclxuICAgICAgICAgICAgICAgIGNhc2UgJ3BsYWluJzpcclxuICAgICAgICAgICAgICAgICAgICBpZiAoaXRlbS5uYW1lLnBsYWluKCkuaW5jbHVkZXMoXCJsb2dcIikpXHJcbiAgICAgICAgICAgICAgICAgICAgICAgIHJldHVybiBgPGRvY3VtZW50LWNvZGUgc3JjPVwiJHtQQUdFX0NPTlRFWFQucmVwb3NfcGF0aCgpfS9maWxlLyR7aXRlbS5pZH1cIiBjbGFzcz1cImxhbmd1YWdlLWxvZ1wiPjwvZG9jdW1lbnQtY29kZT5gXHJcbiAgICAgICAgICAgICAgICAgICAgZWxzZVxyXG4gICAgICAgICAgICAgICAgICAgICAgICByZXR1cm4gYDxkb2N1bWVudC1jb2RlIHNyYz1cIiR7UEFHRV9DT05URVhULnJlcG9zX3BhdGgoKX0vZmlsZS8ke2l0ZW0uaWR9XCIgY2xhc3M9XCJsYW5ndWFnZS1wbGFpblwiPjwvZG9jdW1lbnQtY29kZT5gXHJcbiAgICAgICAgICAgICAgICBjYXNlICdtYXJrZG93bic6XHJcbiAgICAgICAgICAgICAgICBjYXNlICd4LW1hcmtkb3duJzpcclxuICAgICAgICAgICAgICAgICAgICByZXR1cm4gYDxkb2N1bWVudC1tYXJrZG93biBzcmM9XCIke1BBR0VfQ09OVEVYVC5yZXBvc19wYXRoKCl9L2ZpbGUvJHtpdGVtLmlkfVwiPjwvZG9jdW1lbnQtbWFya2Rvd24+YDtcclxuICAgICAgICAgICAgICAgIGNhc2UgJ3Njc3MnOlxyXG4gICAgICAgICAgICAgICAgY2FzZSAneC1zY3NzJzpcclxuICAgICAgICAgICAgICAgICAgICByZXR1cm4gYDxkb2N1bWVudC1jb2RlIHNyYz1cIiR7UEFHRV9DT05URVhULnJlcG9zX3BhdGgoKX0vZmlsZS8ke2l0ZW0uaWR9XCIgY2xhc3M9XCJsYW5ndWFnZS1zY3NzXCI+PC9kb2N1bWVudC1jb2RlPmBcclxuICAgICAgICAgICAgICAgIGNhc2UgJ3Nhc3MnOlxyXG4gICAgICAgICAgICAgICAgY2FzZSAneC1zYXNzJzpcclxuICAgICAgICAgICAgICAgICAgICByZXR1cm4gYDxkb2N1bWVudC1jb2RlIHNyYz1cIiR7UEFHRV9DT05URVhULnJlcG9zX3BhdGgoKX0vZmlsZS8ke2l0ZW0uaWR9XCIgY2xhc3M9XCJsYW5ndWFnZS1zY3NzXCI+PC9kb2N1bWVudC1jb2RlPmBcclxuICAgICAgICAgICAgICAgIGNhc2UgJ2Nzcyc6XHJcbiAgICAgICAgICAgICAgICBjYXNlICd4LWNzcyc6XHJcbiAgICAgICAgICAgICAgICAgICAgcmV0dXJuIGA8ZG9jdW1lbnQtY29kZSBzcmM9XCIke1BBR0VfQ09OVEVYVC5yZXBvc19wYXRoKCl9L2ZpbGUvJHtpdGVtLmlkfVwiIGNsYXNzPVwibGFuZ3VhZ2UtY3NzXCI+PC9kb2N1bWVudC1jb2RlPmBcclxuICAgICAgICAgICAgfVxyXG4gICAgICAgICAgICByZXR1cm4gYDxkb2N1bWVudC1lbWJlZCBzcmM9XCIke1BBR0VfQ09OVEVYVC5yZXBvc19wYXRoKCl9L2ZpbGUvJHtpdGVtLmlkfVwiPjwvZG9jdW1lbnQtZW1iZWQ+YFxyXG4gICAgfVxyXG5cclxuICAgIHJldHVybiBgPGltZyBjbGFzcz1cIml0ZW0tc21hbGxcIiBzcmM9XCIke2dldF9taW1lX2ljb25fcGF0aChpdGVtLm1pbWV0eXBlKX1cIiBhbHQ9XCJkb2N1bWVudDogJHtpdGVtLm5hbWV9XCIvPmA7XHJcbn1cclxuXHJcbmV4cG9ydCB7Z2V0fSIsImltcG9ydCBIYW5kbGViYXJzIGZyb20gXCJoYW5kbGViYXJzXCI7XHJcbmltcG9ydCB7Z2V0X21pbWVfaWNvbl9wYXRoLCBpc19taW1ldHlwZV92YWxpZH0gZnJvbSBcIi4uL2NvbW1vbi90b29scy9taW1lX3V0aWxzXCI7XHJcblxyXG5yZXF1aXJlKCcuL2N1c3RvbV9lbGVtZW50cy9kb2N1bWVudC9jb2RlJylcclxucmVxdWlyZSgnLi9jdXN0b21fZWxlbWVudHMvZG9jdW1lbnQvbWFya2Rvd24nKVxyXG5yZXF1aXJlKCcuL2N1c3RvbV9lbGVtZW50cy9sYXp5X2ltYWdlL2xhenlfaW1hZ2UnKVxyXG5yZXF1aXJlKCcuL2N1c3RvbV9lbGVtZW50cy9wZGZfdmlld2VyL3BkZi12aWV3ZXInKVxyXG5cclxuZnVuY3Rpb24gbWltZV9pbWFnZV9nZW5lcmF0b3JfaGVscGVyX2JpZyhpdGVtKSB7XHJcbiAgICAvLyBDQVNFIDogSVMgRElSRUNUT1JZXHJcbiAgICBpZiAoIWl0ZW0uaXNfcmVndWxhcl9maWxlKSB7XHJcbiAgICAgICAgcmV0dXJuIG5ldyBIYW5kbGViYXJzLlNhZmVTdHJpbmcoYDxpbWcgc3JjPVwiL2ltYWdlcy9pY29ucy9pY29uczgtZm9sZGVyLTk2LnBuZ1wiIGFsdD1cImRvc3NpZXI6ICR7aXRlbS5uYW1lfVwiPmApXHJcbiAgICB9XHJcbiAgICAvLyBDQVNFIDogSVMgU1RBTkRBUkQgRklMRVxyXG4gICAgZWxzZSB7XHJcbiAgICAgICAgaWYgKCFpc19taW1ldHlwZV92YWxpZChpdGVtLm1pbWV0eXBlKSlcclxuICAgICAgICAgICAgcmV0dXJuIG5ldyBIYW5kbGViYXJzLlNhZmVTdHJpbmcoYDxpbWcgY2xhc3M9XCJpdGVtLXNtYWxsXCIgc3JjPVwiJHtnZXRfbWltZV9pY29uX3BhdGgoaXRlbS5taW1ldHlwZSl9XCIgYWx0PVwiZG9jdW1lbnQ6ICR7aXRlbS5uYW1lfVwiLz5gKTtcclxuICAgICAgICAvLyBEaXN0YW50IHJlcG9zXHJcbiAgICAgICAgaWYgKGl0ZW0uaWQpIHtcclxuICAgICAgICAgICAgcmV0dXJuIG5ldyBIYW5kbGViYXJzLlNhZmVTdHJpbmcocmVxdWlyZShcIi4vZGlzdGFudF9yZXBvc1wiKS5nZXQoaXRlbSkpO1xyXG4gICAgICAgIH1cclxuICAgICAgICAvLyBGaWxlc3lzdGVtIGZpbGVcclxuICAgICAgICBlbHNlIGlmIChpdGVtLmxhc3RNb2RpZmllZCkge1xyXG4gICAgICAgICAgICByZXR1cm4gbmV3IEhhbmRsZWJhcnMuU2FmZVN0cmluZyhgPGltZyBjbGFzcz1cIml0ZW0tc21hbGxcIiBzcmM9XCIke2dldF9taW1lX2ljb25fcGF0aChpdGVtLm1pbWV0eXBlKX1cIiBhbHQ9XCJkb2N1bWVudDogJHtpdGVtLm5hbWV9XCIvPmApO1xyXG4gICAgICAgIH1cclxuICAgIH1cclxufVxyXG5cclxuSGFuZGxlYmFycy5yZWdpc3RlckhlbHBlcihcIml0ZW1faW1hZ2VcIiwgKG9wdGlvbnMpID0+IG1pbWVfaW1hZ2VfZ2VuZXJhdG9yX2hlbHBlcl9iaWcob3B0aW9ucykpO1xyXG5cclxuY29uc29sZS5pbmZvKCdMb2FkZWQgdmlld2Vycy5qcycpOyJdLCJuYW1lcyI6WyJMYXp5SW1hZ2UiLCJIVE1MRWxlbWVudCIsImNvbnN0cnVjdG9yIiwic3R5bGUiLCJtYXhXaWR0aCIsIm1heEhlaWdodCIsIm92ZXJmbG93WCIsIm92ZXJmbG93WSIsImhhc0F0dHJpYnV0ZSIsInRtcF9pbWFnZSIsIkltYWdlIiwiY2xhc3NMaXN0IiwiYWRkIiwic3JjIiwiZ2V0QXR0cmlidXRlIiwiYXBwZW5kIiwiaW1hZ2UiLCJvbmxvYWQiLCJpbm5lckhUTUwiLCJjdXN0b21FbGVtZW50cyIsImRlZmluZSIsImdldF9taW1lX2ljb25fcGF0aCIsIlBBR0VfQ09OVEVYVCIsImdldCIsIml0ZW0iLCJ1cmwiLCJyZXBvc19wYXRoIiwiaWQiLCJ0aHVtYm5haWxfdXJsIiwibWltZXR5cGUiLCJwbGFpbiIsInNwbGl0IiwibmFtZSIsImluY2x1ZGVzIiwiSGFuZGxlYmFycyIsImlzX21pbWV0eXBlX3ZhbGlkIiwicmVxdWlyZSIsIm1pbWVfaW1hZ2VfZ2VuZXJhdG9yX2hlbHBlcl9iaWciLCJpc19yZWd1bGFyX2ZpbGUiLCJTYWZlU3RyaW5nIiwibGFzdE1vZGlmaWVkIiwicmVnaXN0ZXJIZWxwZXIiLCJvcHRpb25zIiwiY29uc29sZSIsImluZm8iXSwic291cmNlUm9vdCI6IiJ9