use actix_files::NamedFile;
use actix_web::Responder;
use actix_web::{get, post};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Write;
use std::process::Command;
use util_error::BasicResult;
use util_response::{data, msg, prelude::*};

#[get("/")]
async fn index() -> BasicResult<impl Responder> {
    Ok(NamedFile::open_async("static/browser_editor/index.html")
        .await
        .unwrap())
}

#[derive(Serialize)]
struct ScriptItem {
    file_name: String,
    content: String,
}

#[get("/scripts")]
async fn scripts() -> BasicResult<impl Responder> {
    let data = fs::read_dir("static/scripts")?
        .filter_map(|item| match item {
            Ok(v) => {
                let res = fs::read_to_string(v.path());
                Some(ScriptItem {
                    file_name: v.file_name().into_string().unwrap(),
                    content: res.unwrap(),
                })
            }
            _ => None,
        })
        .collect::<Vec<_>>();

    Ok(Json(data!(data)))
}

#[derive(Deserialize)]
struct SaveReqItem {
    file_name: String,
    content: String,
}

#[post("/save")]
async fn save(req: Json<SaveReqItem>) -> BasicResult<impl Responder> {
    let path = format!("static/scripts/{}", &req.file_name);
    if let Ok(exists) = fs::try_exists(&path) {
        if exists {
            fs::remove_file(&path)?;
        } else {
            return Ok(Json(msg!(format!("file: {} doesn't exists", path))));
        }
    }
    let mut f = File::options().write(true).create_new(true).open(&path)?;
    f.write_fmt(format_args!("{}", &req.content))?;
    Ok(Json(msg!("save successed".to_string())))
}

#[derive(Deserialize)]
struct RunReq {
    content: String,
}

#[post("/run")]
async fn run(req: Json<RunReq>) -> BasicResult<impl Responder> {
    let output = Command::new("nu").arg("-c").arg(&req.content).output()?;
    let res = String::from_utf8_lossy(&output.stdout).to_string();
    let error = String::from_utf8_lossy(&output.stderr).to_string();
    Ok(Json(data!(format!("{}{}", res, error))))
}
