pub mod auth;

#[macro_export]
macro_rules! serve_api {
    ($app: expr) => {
        use crate::api::auth;
        $app = $app.service(
            scope("/api")
                .service(ping)
                .service(
                    scope("/auth")
                        .service(auth::authorize)
                        .service(auth::token)
                        .service(auth::userinfo),
                )
                .service(scope("/custom").wrap(middleware::auth::Auth)),
        );
    };
}
