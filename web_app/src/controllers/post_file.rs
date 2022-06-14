use actix_web::{web, HttpResponse, Responder, ResponseError};
use askama::Template;
use thiserror::Error;
use serde::Deserialize;

#[derive(Template)]
#[template(path="post_file.html")]
struct IndexTemplate{
}

#[derive(Deserialize)]
pub struct GetParameters{
    id: String,
}

#[derive(Error, Debug)]
pub enum MyError {
    #[error("Failed to render HTML")]
    AskamaError(#[from] askama::Error),
}
impl ResponseError for MyError {}

pub async fn index(form: web::Form<GetParameters>) -> Result<impl Responder , MyError>{
    let html = IndexTemplate {};
    let response_body = html.render().unwrap();
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body))
}
