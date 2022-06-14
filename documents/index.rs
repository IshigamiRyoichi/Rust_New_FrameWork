use actix_web::{ HttpResponse, Responder, ResponseError};
use askama::Template;
use thiserror::Error;

// index.htmlのテンプレート定義
#[derive(Template)]
#[template(path="index.html")]
struct IndexTemplate{
    text: String,
}

#[derive(Error, Debug)]
pub enum MyError {
    #[error("Failed to render HTML")]
    AskamaError(#[from] askama::Error),
}
impl ResponseError for MyError {}

pub async fn index() -> Result<impl Responder , MyError>{
    let html = IndexTemplate {text : "Hello World".to_string()};
    let response_body = html.render().unwrap();
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body))
}