const Handlebars = require('handlebars');
const parser = new DOMParser();

// Used to register contexts
if (!document.__handlebar_custom_loader)
    document.__handlebar_custom_loader = {
        __next_obj_id: 0,
        __registered_ctx: {}
    }

Handlebars.get_mime_icons = () => JSON.parse('{{mime_icons}}');

module.exports = (data, ctx) => {
    if (ctx) {
        if (!ctx['__handlebar_ctx_id']) {
            ctx.__handlebar_ctx_id = ++document.__handlebar_custom_loader.__next_obj_id;
            document.__handlebar_custom_loader.__registered_ctx[ctx.__handlebar_ctx_id] = ctx;
        }
        data.__handlebar_ctx_id = ctx.__handlebar_ctx_id;
    }

    const generated_html = Handlebars.template('{{template}}')(data);
    const body = parser.parseFromString(generated_html, 'text/html').body;
    if (body.children.length === 1) {
        return body.children[0];
    }

    // Force children generation
    const children = [];
    for (let i = 0; i < body.children.length; ++i) {
        children.push(body.children[i]);
    }
    return children;
}