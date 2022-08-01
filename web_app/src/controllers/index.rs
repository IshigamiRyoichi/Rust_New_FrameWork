use actix_web::{web, HttpResponse, Responder, ResponseError};
use askama::Template;
use thiserror::Error;
use serde::Deserialize;

// index.htmlのテンプレート定義
#[derive(Template)]
#[template(path="view/index.html")]
struct IndexTemplate{
    text: String,
}

// 値取得用の構造体
#[derive(Deserialize)]
pub struct GetParameters{
    id: String,
    pass: String,
}

#[derive(Error, Debug)]
pub enum MyError {
    #[error("Failed to render HTML")]
    AskamaError(#[from] askama::Error),
}
impl ResponseError for MyError {}

pub async fn index(form: web::Form<GetParameters>) -> Result<impl Responder , MyError>{
    let id = &form.id;
    let text = format!("Hello {}!!",id);
    let html = IndexTemplate {text : text};
    let response_body = html.render().unwrap();
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body))
}