touch ./src/controllers/$1.rs
touch ./templates/$1.html
# ROUTE='    cfg.route("/index", web::get().to('$1'::'$1'));'
# tac seq.txt | sed '5i '$ROUTE | tac > seq2.txt

echo 'use actix_web::{HttpResponse, Responder, ResponseError};
use askama::Template;
use thiserror::Error;

#[derive(Template)]
#[template(path="'$1'.html")]
struct IndexTemplate{
}

#[derive(Error, Debug)]
pub enum MyError {
    #[error("Failed to render HTML")]
    AskamaError(#[from] askama::Error),
}
impl ResponseError for MyError {}

pub async fn login() -> Result<impl Responder, MyError>{
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
    <link rel="stylesheet" href="css/style.css">
    <title></title>
</head>
<body>
</body>
</html>' > ./templates/$1.html

echo 'pub mod '$1';' >> ./src/controllers/mod.rs