use actix_web::{web, App, HttpResponse, HttpServer, Responder, ResponseError};
use actix_files::Files;
use askama::Template;
use thiserror::Error;

#[derive(Template)]
#[template(path="model/model.html")]
struct IndexTemplate{
    text: String,
}

#[derive(Error, Debug)]
pub enum MyError {
    #[error("Failed to render HTML")]
    AskamaError(#[from] askama::Error),
}
impl ResponseError for MyError {}

pub async fn model() -> Result<impl Responder, MyError>{
    let html = IndexTemplate {text : "Please Creating Model".to_string()};
    let response_body = html.render()?;
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("     access:http://127.0.0.1:5000/model");
    HttpServer::new(|| App::new()
        .service(Files::new("/css", "./css").show_files_listing())
        .service(Files::new("/static", "./static").show_files_listing()) 
        .service(Files::new("/test_model", "./test_model").show_files_listing())  
        .route("/model", web::get().to(model))
        )
        .bind("127.0.0.1:5000")?
        .run()
        .await
}