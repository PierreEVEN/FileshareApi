import {APP_CONFIG} from "../types/app_config";
import {APP_COOKIES} from "./cookies";
import {Authentication} from "../modules/authentication/authentication";
import {EncString} from "../types/encstring";

/**
 * @param path
 * @param method
 * @param body
 * @returns {Promise<object|object[]|any>}
 */
async function fetch_api(path, method = 'GET', body = null) {
    const headers = new Headers();
    if (body)
        headers.append('Content-Type', 'application/json');
    headers.append('Accept', 'application/json');
    headers.append('content-authtoken', APP_COOKIES.get_token());
    const result = await fetch(`${APP_CONFIG.origin()}/api/${path}`, {
        method: method,
        body: body ? JSON.stringify(body) : null,
        headers: headers
    });
    if (result.status === 401) {
        await Authentication.login();
    } else {
        if (result.status.toString().startsWith("2")) {
            let text = await result.text();
            try {
                return JSON.parse(text);
            }
            catch (err) {
                return text;
            }
        }
    }
    throw {message: `${await result.text()}`, code:result.status}
}

async function fetch_user(path, method = 'GET', body = null) {
    return await fetch_api(`${APP_CONFIG.display_user().name.plain()}/${path}`, method, body)
}

async function fetch_repository(path, method = 'GET', body = null) {
    return await fetch_user(`${APP_CONFIG.display_repository().url_name.plain()}/${path}`, method, body)
}

export {fetch_api, fetch_user, fetch_repository}