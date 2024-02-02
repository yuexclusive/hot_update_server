pub mod sync;
pub mod security;
pub mod auth;

use utoipa::OpenApi;
use utoipa_swagger_ui::{SwaggerUi, Url};

#[macro_export]
macro_rules! serve_openapi {
    ($app: expr) => {
        $app = $app.service(crate::openapi::swaggerui());
    };
}

pub fn swaggerui() -> SwaggerUi {
    // app = app.service(SwaggerUi::new("/swagger/user/{_:.*}").url(
    //     "/api-doc/user.json",
    //     openapi::user::ApiDoc::openapi().clone(),
    // ))
    let urls = vec![
        (
            Url::new("auth", "/api-doc/auth.json"),
            crate::openapi::auth::ApiDoc::openapi().clone(),
        ),
        (
            Url::new("sync", "/api-doc/sync.json"),
            crate::openapi::sync::ApiDoc::openapi().clone(),
        ),
    ];
    SwaggerUi::new("/swagger/{_:.*}").urls(urls)
}
