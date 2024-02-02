use actix_web::web::{Json, Query};
use actix_web::{get, Responder};
use std::process::Command;
use util_error::BasicResult;
use util_response::{msg, prelude::*};
use utoipa::ToSchema;

use serde::{Deserialize, Serialize};

#[derive(utoipa::IntoParams, Deserialize, Serialize)]
pub struct AuthorizeQuery {
    #[param(example = "code")]
    pub response_type: Option<String>,
    #[param(example = "xxxxx")]
    pub app_id: Option<String>,
    #[param(example = "https://www.google.com")]
    pub redirect_uri: String,
    #[param(example = "scope")]
    pub scope: Option<String>,
    #[param(example = "1")]
    pub state: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct AuthorizeResponse {
    pub data: String,
}

#[utoipa::path(
    path = "/api/auth/authorize",
    params(
        AuthorizeQuery
    ),
    responses(
        (status = 200, description = "successfully", body = AuthorizeResponse),
        (status = 400, description = "bad request", body = MsgResponseWithErrCode),
        (status = 500, description = "internal server error", body = MsgResponseWithErrCode)
    )
)]
#[get("/authorize")]
pub async fn authorize(req: Query<AuthorizeQuery>) -> BasicResult<impl Responder> {
    let mut cmd = Command::new("nu");
    let str = serde_json::to_string(&req.0).unwrap();
    let output = cmd.args(["static/scripts/authorize.nu", &str]).output()?;
    let res = String::from_utf8_lossy(&output.stdout)
        .trim_matches('\n')
        .to_string();

    println!("res : {}", res);
    Ok(redirect("/", res))
    // msg!("ok")
}

#[derive(utoipa::IntoParams, Deserialize)]
pub struct TokenQuery {
    pub code: Option<String>,
    pub timestamp: Option<usize>,
}

#[utoipa::path(
    path = "/api/auth/token",
    params(
        TokenQuery
    ),
    responses(
        (status = 200, description = "successfully", body = MsgResponse),
        (status = 400, description = "bad request", body = MsgResponseWithErrCode),
        (status = 500, description = "internal server error", body = MsgResponseWithErrCode)
    )
)]
#[get("/token")]
pub async fn token(_req: Query<TokenQuery>) -> BasicResult<impl Responder> {
    // user_service::validate_exist_email(&email.into_inner()).await?;
    Ok(Json(msg!("token")))
}

#[derive(utoipa::IntoParams, Deserialize)]
pub struct UserinfoQuery {
    pub code: String,
    pub timestamp: usize,
}

#[derive(Serialize, Default)]
pub struct UserInfo {
    pub name: String,
    pub age: u8,
}

#[utoipa::path(
    path = "/api/auth/userinfo",
    params(
        UserinfoQuery
    ),
    responses(
        (status = 200, description = "successfully", body = MsgResponse),
        (status = 400, description = "bad request", body = MsgResponseWithErrCode),
        (status = 500, description = "internal server error", body = MsgResponseWithErrCode)
    )
)]
#[get("/userinfo")]
pub async fn userinfo(_req: Query<TokenQuery>) -> BasicResult<impl Responder> {
    // user_service::validate_exist_email(&email.into_inner()).await?;
    // let a = ;
    // Ok(actix_web::HttpResponse::Ok().body(Json(msg!("test")).into()))

    Ok(Json(msg!("userinfo")))
}
