#![allow(dead_code)]
#![feature(fs_try_exists)]

mod api;
mod config;
mod editor;
mod init;
mod middleware;
mod openapi;
mod upload_file;

use actix_web::{get, web::scope, App, HttpServer, Result};
use util_error::ErrorKind;

#[get("/ping")]
pub async fn ping() -> &'static str {
    "pong"
}

#[actix_web::main]
async fn main() -> Result<(), ErrorKind> {
    init::init().await?;

    HttpServer::new(move || {
        let mut app = App::new()
            .wrap(middleware::logger::logger())
            .wrap(middleware::cors::cors());

        serve_api!(app);
        serve_openapi!(app);
        serve_upload_file!(app);
        serve_editor!(app);
        app
    })
    .bind((config::cfg().host.as_str(), config::cfg().port))?
    .run()
    .await
    .unwrap();

    log::info!("server stoped");
    Ok(())
}

#[cfg(test)]
mod tests {
    use actix_web::{body, test, web, App};

    use super::*;

    #[actix_web::test]
    async fn test_index_get() -> Result<(), Box<dyn std::error::Error>> {
        let app = test::init_service(App::new().service(ping)).await;
        let req = test::TestRequest::get().uri("/ping").to_request();
        let resp = test::call_service(&app, req).await;
        let bytes = body::to_bytes(resp.into_body()).await?;
        assert_eq!(bytes, web::Bytes::from_static(b"pong"));
        Ok(())
    }
}
