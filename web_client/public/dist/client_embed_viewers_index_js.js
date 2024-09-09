(self["webpackChunkevenpierre"] = self["webpackChunkevenpierre"] || []).push([["client_embed_viewers_index_js"],{

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

}]);
//# sourceMappingURL=data:application/json;charset=utf-8;base64,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Y2s6Ly9ldmVucGllcnJlLy4vY2xpZW50L2VtYmVkX3ZpZXdlcnMvY3VzdG9tX2VsZW1lbnRzL2xhenlfaW1hZ2UvbGF6eV9pbWFnZS5qcyIsIndlYnBhY2s6Ly9ldmVucGllcnJlLy4vY2xpZW50L2VtYmVkX3ZpZXdlcnMvZGlzdGFudF9yZXBvcy5qcyIsIndlYnBhY2s6Ly9ldmVucGllcnJlLy4vY2xpZW50L2VtYmVkX3ZpZXdlcnMvaW5kZXguanMiXSwic291cmNlc0NvbnRlbnQiOlsiY2xhc3MgTGF6eUltYWdlIGV4dGVuZHMgSFRNTEVsZW1lbnQge1xyXG4gICAgY29uc3RydWN0b3IoKSB7XHJcbiAgICAgICAgc3VwZXIoKTtcclxuXHJcbiAgICAgICAgdGhpcy5zdHlsZS5tYXhXaWR0aCA9ICcxMDAlJztcclxuICAgICAgICB0aGlzLnN0eWxlLm1heEhlaWdodCA9ICcxMDAlJztcclxuICAgICAgICB0aGlzLnN0eWxlLm92ZXJmbG93WCA9ICdoaWRkZW4nO1xyXG4gICAgICAgIHRoaXMuc3R5bGUub3ZlcmZsb3dZID0gJ2hpZGRlbic7XHJcblxyXG4gICAgICAgIGlmICghdGhpcy5oYXNBdHRyaWJ1dGUoJ3NyYycpKVxyXG4gICAgICAgICAgICByZXR1cm47XHJcblxyXG4gICAgICAgIGlmICh0aGlzLmhhc0F0dHJpYnV0ZSgnYWx0ZXJuYXRlLXNyYycpKSB7XHJcbiAgICAgICAgICAgIGNvbnN0IHRtcF9pbWFnZSA9IG5ldyBJbWFnZSgpO1xyXG4gICAgICAgICAgICB0bXBfaW1hZ2UuY2xhc3NMaXN0LmFkZCgnaXRlbS1sYXJnZScpO1xyXG4gICAgICAgICAgICB0bXBfaW1hZ2Uuc3JjID0gdGhpcy5nZXRBdHRyaWJ1dGUoJ2FsdGVybmF0ZS1zcmMnKTtcclxuICAgICAgICAgICAgdGhpcy5hcHBlbmQodG1wX2ltYWdlKTtcclxuICAgICAgICB9XHJcbiAgICAgICAgLy9vbkVycm9yPVwidGhpcy5vbkVycm9yID0gbnVsbDsgdGhpcy5zcmM9Jy9pbWFnZXMvaWNvbnMvbWltZS1pY29ucy9pbWFnZS5wbmcnXCJcclxuICAgICAgICBjb25zdCBpbWFnZSA9IG5ldyBJbWFnZSgpO1xyXG4gICAgICAgIGltYWdlLnNyYyA9IHRoaXMuZ2V0QXR0cmlidXRlKCdzcmMnKTtcclxuICAgICAgICBpbWFnZS5jbGFzc0xpc3QuYWRkKCdpdGVtLWxhcmdlJyk7XHJcbiAgICAgICAgaW1hZ2Uub25sb2FkID0gKCkgPT4ge1xyXG4gICAgICAgICAgICB0aGlzLmlubmVySFRNTCA9ICcnO1xyXG4gICAgICAgICAgICB0aGlzLmFwcGVuZChpbWFnZSk7XHJcbiAgICAgICAgfVxyXG4gICAgfVxyXG5cclxufVxyXG5cclxuY3VzdG9tRWxlbWVudHMuZGVmaW5lKFwibGF6eS1pbWdcIiwgTGF6eUltYWdlKTsiLCIgICAgaW1wb3J0IHtnZXRfbWltZV9pY29uX3BhdGh9IGZyb20gXCIuLi9jb21tb24vdG9vbHMvbWltZV91dGlsc1wiO1xyXG5pbXBvcnQge1BBR0VfQ09OVEVYVH0gZnJvbSBcIi4uL2NvbW1vbi90b29scy91dGlsc1wiO1xyXG5cclxuZnVuY3Rpb24gZ2V0KGl0ZW0pIHtcclxuICAgIGNvbnN0IHVybCA9IGAke1BBR0VfQ09OVEVYVC5yZXBvc19wYXRoKCl9L2ZpbGUvJHtpdGVtLmlkfWA7XHJcbiAgICBjb25zdCB0aHVtYm5haWxfdXJsID0gYCR7UEFHRV9DT05URVhULnJlcG9zX3BhdGgoKX0vdGh1bWJuYWlsLyR7aXRlbS5pZH1gO1xyXG4gICAgY29uc3QgbWltZXR5cGUgPSBpdGVtLm1pbWV0eXBlLnBsYWluKCkuc3BsaXQoJy8nKTtcclxuICAgIHN3aXRjaCAobWltZXR5cGVbMF0pIHtcclxuICAgICAgICBjYXNlICdpbWFnZSc6XHJcbiAgICAgICAgICAgIHJldHVybiBgPGxhenktaW1nIGNsYXNzPVwiaXRlbS1sYXJnZVwiIHNyYz1cIiR7dXJsfVwiIGFsdGVybmF0ZS1zcmM9XCIke3RodW1ibmFpbF91cmx9XCJcIi8+YFxyXG4gICAgICAgIGNhc2UgJ3ZpZGVvJzpcclxuICAgICAgICAgICAgcmV0dXJuIGA8dmlkZW8gY2xhc3M9XCJpdGVtLWxhcmdlIHZpZGVvLWpzXCIgcHJlbG9hZD1cImF1dG9cIiBkYXRhLXNldHVwPVwie31cIiBhdXRvcGxheT1cInRydWVcIiBwcmVsb2FkPVwiYXV0b1wiIGNvbnRyb2xzPVwidHJ1ZVwiIGhlaWdodD1cIjEwMCVcIiB3aWR0aD1cIjEwMCVcIj5cclxuICAgICAgICAgICAgICAgICAgICAgICAgPHNvdXJjZSBzcmM9XCIke3VybH1cIiB0eXBlPVwiJHtpdGVtLm1pbWV0eXBlfVwiPlxyXG4gICAgICAgICAgICAgICAgICAgIDwvdmlkZW8+YFxyXG4gICAgICAgIGNhc2UgJ2F1ZGlvJzpcclxuICAgICAgICAgICAgcmV0dXJuIGA8YXVkaW8gY29udHJvbHM9XCJ0cnVlXCIgc3JjPVwiJHtQQUdFX0NPTlRFWFQucmVwb3NfcGF0aCgpfS9maWxlLyR7aXRlbS5pZH1cIj48L2F1ZGlvPmBcclxuICAgICAgICBjYXNlICdhcHBsaWNhdGlvbic6XHJcbiAgICAgICAgICAgIHN3aXRjaCAobWltZXR5cGVbMV0pIHtcclxuICAgICAgICAgICAgICAgIGNhc2UgJ3gtcGRmJzpcclxuICAgICAgICAgICAgICAgIGNhc2UgJ3BkZic6XHJcbiAgICAgICAgICAgICAgICAgICAgcmV0dXJuIGA8b2JqZWN0IGRhdGE9XCIke3VybH1cIiB0eXBlPVwiYXBwbGljYXRpb24vcGRmXCIgd2lkdGg9XCIxMDAlXCIgaGVpZ2h0PVwiMTAwJVwiPlxyXG4gICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgIDxwZGYtZW1iZWQgc3JjPVwiJHt1cmx9XCI+PC9wZGYtZW1iZWQ+XHJcbiAgICAgICAgICAgICAgICAgICAgICAgICAgICA8L29iamVjdD5gXHJcbiAgICAgICAgICAgICAgICBjYXNlICdqc29uJzpcclxuICAgICAgICAgICAgICAgIGNhc2UgJ3gtanNvbic6XHJcbiAgICAgICAgICAgICAgICAgICAgcmV0dXJuIGA8ZG9jdW1lbnQtY29kZSBzcmM9XCIke1BBR0VfQ09OVEVYVC5yZXBvc19wYXRoKCl9L2ZpbGUvJHtpdGVtLmlkfVwiIGNsYXNzPVwibGFuZ3VhZ2UtanNvblwiPjwvZG9jdW1lbnQtY29kZT5gXHJcbiAgICAgICAgICAgICAgICBjYXNlICdqYXZhc2NyaXB0JzpcclxuICAgICAgICAgICAgICAgIGNhc2UgJ3gtamF2YXNjcmlwdCc6XHJcbiAgICAgICAgICAgICAgICAgICAgcmV0dXJuIGA8ZG9jdW1lbnQtY29kZSBzcmM9XCIke1BBR0VfQ09OVEVYVC5yZXBvc19wYXRoKCl9L2ZpbGUvJHtpdGVtLmlkfVwiIGNsYXNzPVwibGFuZ3VhZ2UtanNcIj48L2RvY3VtZW50LWNvZGU+YFxyXG4gICAgICAgICAgICB9XHJcbiAgICAgICAgICAgIGJyZWFrO1xyXG4gICAgICAgIGNhc2UgJ3RleHQnOlxyXG4gICAgICAgICAgICBzd2l0Y2ggKG1pbWV0eXBlWzFdKSB7XHJcbiAgICAgICAgICAgICAgICBjYXNlICdwbGFpbic6XHJcbiAgICAgICAgICAgICAgICAgICAgaWYgKGl0ZW0ubmFtZS5wbGFpbigpLmluY2x1ZGVzKFwibG9nXCIpKVxyXG4gICAgICAgICAgICAgICAgICAgICAgICByZXR1cm4gYDxkb2N1bWVudC1jb2RlIHNyYz1cIiR7UEFHRV9DT05URVhULnJlcG9zX3BhdGgoKX0vZmlsZS8ke2l0ZW0uaWR9XCIgY2xhc3M9XCJsYW5ndWFnZS1sb2dcIj48L2RvY3VtZW50LWNvZGU+YFxyXG4gICAgICAgICAgICAgICAgICAgIGVsc2VcclxuICAgICAgICAgICAgICAgICAgICAgICAgcmV0dXJuIGA8ZG9jdW1lbnQtY29kZSBzcmM9XCIke1BBR0VfQ09OVEVYVC5yZXBvc19wYXRoKCl9L2ZpbGUvJHtpdGVtLmlkfVwiIGNsYXNzPVwibGFuZ3VhZ2UtcGxhaW5cIj48L2RvY3VtZW50LWNvZGU+YFxyXG4gICAgICAgICAgICAgICAgY2FzZSAnbWFya2Rvd24nOlxyXG4gICAgICAgICAgICAgICAgY2FzZSAneC1tYXJrZG93bic6XHJcbiAgICAgICAgICAgICAgICAgICAgcmV0dXJuIGA8ZG9jdW1lbnQtbWFya2Rvd24gc3JjPVwiJHtQQUdFX0NPTlRFWFQucmVwb3NfcGF0aCgpfS9maWxlLyR7aXRlbS5pZH1cIj48L2RvY3VtZW50LW1hcmtkb3duPmA7XHJcbiAgICAgICAgICAgICAgICBjYXNlICdzY3NzJzpcclxuICAgICAgICAgICAgICAgIGNhc2UgJ3gtc2Nzcyc6XHJcbiAgICAgICAgICAgICAgICAgICAgcmV0dXJuIGA8ZG9jdW1lbnQtY29kZSBzcmM9XCIke1BBR0VfQ09OVEVYVC5yZXBvc19wYXRoKCl9L2ZpbGUvJHtpdGVtLmlkfVwiIGNsYXNzPVwibGFuZ3VhZ2Utc2Nzc1wiPjwvZG9jdW1lbnQtY29kZT5gXHJcbiAgICAgICAgICAgICAgICBjYXNlICdzYXNzJzpcclxuICAgICAgICAgICAgICAgIGNhc2UgJ3gtc2Fzcyc6XHJcbiAgICAgICAgICAgICAgICAgICAgcmV0dXJuIGA8ZG9jdW1lbnQtY29kZSBzcmM9XCIke1BBR0VfQ09OVEVYVC5yZXBvc19wYXRoKCl9L2ZpbGUvJHtpdGVtLmlkfVwiIGNsYXNzPVwibGFuZ3VhZ2Utc2Nzc1wiPjwvZG9jdW1lbnQtY29kZT5gXHJcbiAgICAgICAgICAgICAgICBjYXNlICdjc3MnOlxyXG4gICAgICAgICAgICAgICAgY2FzZSAneC1jc3MnOlxyXG4gICAgICAgICAgICAgICAgICAgIHJldHVybiBgPGRvY3VtZW50LWNvZGUgc3JjPVwiJHtQQUdFX0NPTlRFWFQucmVwb3NfcGF0aCgpfS9maWxlLyR7aXRlbS5pZH1cIiBjbGFzcz1cImxhbmd1YWdlLWNzc1wiPjwvZG9jdW1lbnQtY29kZT5gXHJcbiAgICAgICAgICAgIH1cclxuICAgICAgICAgICAgcmV0dXJuIGA8ZG9jdW1lbnQtZW1iZWQgc3JjPVwiJHtQQUdFX0NPTlRFWFQucmVwb3NfcGF0aCgpfS9maWxlLyR7aXRlbS5pZH1cIj48L2RvY3VtZW50LWVtYmVkPmBcclxuICAgIH1cclxuXHJcbiAgICByZXR1cm4gYDxpbWcgY2xhc3M9XCJpdGVtLXNtYWxsXCIgc3JjPVwiJHtnZXRfbWltZV9pY29uX3BhdGgoaXRlbS5taW1ldHlwZSl9XCIgYWx0PVwiZG9jdW1lbnQ6ICR7aXRlbS5uYW1lfVwiLz5gO1xyXG59XHJcblxyXG5leHBvcnQge2dldH0iLCJpbXBvcnQgSGFuZGxlYmFycyBmcm9tIFwiaGFuZGxlYmFyc1wiO1xyXG5pbXBvcnQge2dldF9taW1lX2ljb25fcGF0aCwgaXNfbWltZXR5cGVfdmFsaWR9IGZyb20gXCIuLi9jb21tb24vdG9vbHMvbWltZV91dGlsc1wiO1xyXG5cclxucmVxdWlyZSgnLi9jdXN0b21fZWxlbWVudHMvZG9jdW1lbnQvY29kZScpXHJcbnJlcXVpcmUoJy4vY3VzdG9tX2VsZW1lbnRzL2RvY3VtZW50L21hcmtkb3duJylcclxucmVxdWlyZSgnLi9jdXN0b21fZWxlbWVudHMvbGF6eV9pbWFnZS9sYXp5X2ltYWdlJylcclxucmVxdWlyZSgnLi9jdXN0b21fZWxlbWVudHMvcGRmX3ZpZXdlci9wZGYtdmlld2VyJylcclxuXHJcbmZ1bmN0aW9uIG1pbWVfaW1hZ2VfZ2VuZXJhdG9yX2hlbHBlcl9iaWcoaXRlbSkge1xyXG4gICAgLy8gQ0FTRSA6IElTIERJUkVDVE9SWVxyXG4gICAgaWYgKCFpdGVtLmlzX3JlZ3VsYXJfZmlsZSkge1xyXG4gICAgICAgIHJldHVybiBuZXcgSGFuZGxlYmFycy5TYWZlU3RyaW5nKGA8aW1nIHNyYz1cIi9pbWFnZXMvaWNvbnMvaWNvbnM4LWZvbGRlci05Ni5wbmdcIiBhbHQ9XCJkb3NzaWVyOiAke2l0ZW0ubmFtZX1cIj5gKVxyXG4gICAgfVxyXG4gICAgLy8gQ0FTRSA6IElTIFNUQU5EQVJEIEZJTEVcclxuICAgIGVsc2Uge1xyXG4gICAgICAgIGlmICghaXNfbWltZXR5cGVfdmFsaWQoaXRlbS5taW1ldHlwZSkpXHJcbiAgICAgICAgICAgIHJldHVybiBuZXcgSGFuZGxlYmFycy5TYWZlU3RyaW5nKGA8aW1nIGNsYXNzPVwiaXRlbS1zbWFsbFwiIHNyYz1cIiR7Z2V0X21pbWVfaWNvbl9wYXRoKGl0ZW0ubWltZXR5cGUpfVwiIGFsdD1cImRvY3VtZW50OiAke2l0ZW0ubmFtZX1cIi8+YCk7XHJcbiAgICAgICAgLy8gRGlzdGFudCByZXBvc1xyXG4gICAgICAgIGlmIChpdGVtLmlkKSB7XHJcbiAgICAgICAgICAgIHJldHVybiBuZXcgSGFuZGxlYmFycy5TYWZlU3RyaW5nKHJlcXVpcmUoXCIuL2Rpc3RhbnRfcmVwb3NcIikuZ2V0KGl0ZW0pKTtcclxuICAgICAgICB9XHJcbiAgICAgICAgLy8gRmlsZXN5c3RlbSBmaWxlXHJcbiAgICAgICAgZWxzZSBpZiAoaXRlbS5sYXN0TW9kaWZpZWQpIHtcclxuICAgICAgICAgICAgcmV0dXJuIG5ldyBIYW5kbGViYXJzLlNhZmVTdHJpbmcoYDxpbWcgY2xhc3M9XCJpdGVtLXNtYWxsXCIgc3JjPVwiJHtnZXRfbWltZV9pY29uX3BhdGgoaXRlbS5taW1ldHlwZSl9XCIgYWx0PVwiZG9jdW1lbnQ6ICR7aXRlbS5uYW1lfVwiLz5gKTtcclxuICAgICAgICB9XHJcbiAgICB9XHJcbn1cclxuXHJcbkhhbmRsZWJhcnMucmVnaXN0ZXJIZWxwZXIoXCJpdGVtX2ltYWdlXCIsIChvcHRpb25zKSA9PiBtaW1lX2ltYWdlX2dlbmVyYXRvcl9oZWxwZXJfYmlnKG9wdGlvbnMpKTtcclxuXHJcbmNvbnNvbGUuaW5mbygnTG9hZGVkIHZpZXdlcnMuanMnKTsiXSwibmFtZXMiOlsiTGF6eUltYWdlIiwiSFRNTEVsZW1lbnQiLCJjb25zdHJ1Y3RvciIsInN0eWxlIiwibWF4V2lkdGgiLCJtYXhIZWlnaHQiLCJvdmVyZmxvd1giLCJvdmVyZmxvd1kiLCJoYXNBdHRyaWJ1dGUiLCJ0bXBfaW1hZ2UiLCJJbWFnZSIsImNsYXNzTGlzdCIsImFkZCIsInNyYyIsImdldEF0dHJpYnV0ZSIsImFwcGVuZCIsImltYWdlIiwib25sb2FkIiwiaW5uZXJIVE1MIiwiY3VzdG9tRWxlbWVudHMiLCJkZWZpbmUiLCJnZXRfbWltZV9pY29uX3BhdGgiLCJQQUdFX0NPTlRFWFQiLCJnZXQiLCJpdGVtIiwidXJsIiwicmVwb3NfcGF0aCIsImlkIiwidGh1bWJuYWlsX3VybCIsIm1pbWV0eXBlIiwicGxhaW4iLCJzcGxpdCIsIm5hbWUiLCJpbmNsdWRlcyIsIkhhbmRsZWJhcnMiLCJpc19taW1ldHlwZV92YWxpZCIsInJlcXVpcmUiLCJtaW1lX2ltYWdlX2dlbmVyYXRvcl9oZWxwZXJfYmlnIiwiaXNfcmVndWxhcl9maWxlIiwiU2FmZVN0cmluZyIsImxhc3RNb2RpZmllZCIsInJlZ2lzdGVySGVscGVyIiwib3B0aW9ucyIsImNvbnNvbGUiLCJpbmZvIl0sInNvdXJjZVJvb3QiOiIifQ==