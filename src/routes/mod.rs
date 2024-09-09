pub mod root;
pub mod user;
pub mod repository;



#[macro_export]
macro_rules! get_connected_user {
    ($request:expr, $prop:ident, $body:expr, $or_else:expr) => {{
        let req_ctx = $request.extensions().get::<Arc<RequestContext>>().unwrap();
        if let Some($prop) = req_ctx.connected_user().await.as_ref() {
            {$body}
        } else {
            $or_else
        }
    }};

    ($request:expr, $prop:ident, $body:expr) => (
        let req_ctx = $request.extensions().get::<Arc<RequestContext>>().unwrap();
        if let Some($prop) = req_ctx.connected_user().await.as_ref() {
            $body
        }
    );
}

#[macro_export]
macro_rules! require_connected_user {
    ($request:expr) => {{
        crate::get_connected_user!($request, connected_user, {
            connected_user.clone()
        }, {
            return Err(ServerError::msg(StatusCode::UNAUTHORIZED, "Not connected"))
        })
    }};
}


#[macro_export]
macro_rules! get_display_repository {
    ($request:expr, $prop:ident, $body:expr, $or_else:expr) => {{
        let req_ctx = $request.extensions().get::<Arc<RequestContext>>().unwrap();
        if let Some($prop) = req_ctx.display_repository().await.as_ref() {
            {$body}
        } else {
            $or_else
        }
    }};

    ($request:expr, $prop:ident, $body:expr) => (
        let req_ctx = $request.extensions().get::<Arc<RequestContext>>().unwrap();
        if let Some($prop) = req_ctx.display_repository().await.as_ref() {
            $body
        }
    );
}

#[macro_export]
macro_rules! require_display_repository {
    ($request:expr) => {{
        crate::get_display_repository!($request, display_directory, {
            display_directory.clone()
        }, {
            return Err(ServerError::msg(StatusCode::UNAUTHORIZED, "Invalid repository"))
        })
    }};
}