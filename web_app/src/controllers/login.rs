use actix_web::{HttpResponse, Responder, ResponseError};
use askama::Template;
use thiserror::Error;

#[derive(Template)]
#[template(path="view/login.html")]
struct IndexTemplate{
    text: String,
}

#[derive(Error, Debug)]
pub enum MyError {
    #[error("Failed to render HTML")]
    AskamaError(#[from] askama::Error),
}
impl ResponseError for MyError {}

pub async fn login() -> Result<impl Responder, MyError>{
    let html = IndexTemplate {text : "Please LogIn".to_string()};
    let response_body = html.render().unwrap();
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body))
}