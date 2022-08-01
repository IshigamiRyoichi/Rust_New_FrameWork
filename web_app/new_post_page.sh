touch ./src/controllers/$1.rs
touch ./templates/$1.html

echo 'use actix_web::{web, HttpResponse, Responder, ResponseError};
use askama::Template;
use thiserror::Error;
use serde::Deserialize;

#[derive(Template)]
#[template(path="view/'$1'.html")]
struct IndexTemplate{
}

#[derive(Deserialize)]
pub struct GetParameters{
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
}' > ./src/controllers/$1.rs
echo '<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8" />
    <link rel="stylesheet" href="../../css/view/style.css">
    <title></title>
</head>
<body>
</body>
</html>' > ./templates/view/$1.html

echo 'pub mod '$1';' >> ./src/controllers/mod.rs