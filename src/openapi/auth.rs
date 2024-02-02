use crate::api::auth as auth_controller;
use crate::openapi::security::SecurityAddon;
use util_response::{MsgResponse, MsgResponseWithErrCode};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        auth_controller::authorize,
        auth_controller::token,
        auth_controller::userinfo,
    ),
    components(
        schemas(
            MsgResponse,
            MsgResponseWithErrCode,
            auth_controller::AuthorizeResponse,
        )
    ),
    tags(
        (name = "auth", description = "auth by third party")
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;
