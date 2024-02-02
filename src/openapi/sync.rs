use crate::openapi::security::SecurityAddon;
use util_response::MsgResponse;
use utoipa::OpenApi;
#[derive(OpenApi)]
#[openapi(
    paths(
    ),
    components(
        schemas(
            MsgResponse,
        )
    ),
    tags(
        (name = "sync", description = "user and dept sync")
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;
