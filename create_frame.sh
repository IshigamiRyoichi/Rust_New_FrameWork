cargo new $1
mkdir ./$1/src/controllers
mkdir ./$1/templates
mkdir ./$1/model
mkdir ./$1/model/db
mkdir ./$1/css
mkdir ./$1/test
touch ./$1/css/style.css
name = $1

echo '<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8" />
    <link rel="stylesheet" href="css/style.css">
    <title>App</title>
</head>
<body>
    <div>
        <h1>{{text}}</h1>
    </div>
</body>
</html>' > ./$1/templates/index.html

echo 'use actix_web::{App, HttpServer};
use '$1'::routes;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(routes::routes))
        .bind("localhost:8000")?
        .run()
        .await
}' > ./$1/src/main.rs

echo 'use actix_web::{ HttpResponse, Responder, ResponseError};
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
}' > ./$1/src/controllers/index.rs

echo 'pub mod index;' > ./$1/src/controllers/mod.rs

echo 'pub mod routes;
pub mod controllers;' > ./$1/src/lib.rs

echo 'use actix_web::web;
use crate::controllers::*;
pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(index::index));
}' > ./$1/src/routes.rs

echo 'use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::collections::BTreeMap;
#[derive(Serialize, Deserialize, Debug)]
struct Config {
    databases: BTreeMap <String, BTreeMap <String, BTreeMap <String, String>>>
}
fn read_json() -> BTreeMap <String, BTreeMap <String, BTreeMap <String, String>>>{
    // fn read_json(){
        let json_file_name = "./model/databases.json";
        let json_file = File::open(json_file_name).unwrap();
        let reader_json = BufReader::new(json_file);
        let _config_json : Config = serde_json::from_reader(reader_json).unwrap();
        println!("{:?}",_config_json);
        _config_json.databases
    }
    
fn connection_DB_and_create_Table(db_name : &String, create_table : String) -> Result<()>{
    let cn = Connection::open(db_name)?;
    cn.execute(&create_table, params![])?;
    println!("Create tables.");
    // println!("{:?}",create_table);
    Ok(())
}
fn process_json(config_json_databases : BTreeMap <String, BTreeMap <String, BTreeMap<String,String>>>) {
    for (sql,tables) in config_json_databases.iter() {
        let db_name = format!("./model/db/{}.db", sql);
        for (table, colums) in tables{
            let mut create_table = format!("CREATE TABLE {} (", table);
            for (colum,type_colum) in colums{
                create_table = format!("{}{} {},", create_table, colum, type_colum); 
            }
            create_table.pop();
            create_table = create_table + ")";
            // println!("{:?}",create_table);
            connection_DB_and_create_Table(&db_name, create_table);
        }
    }
}
fn main() {
    let config_json_databases;
    println!("Hello World!");
    config_json_databases = read_json();
    process_json(config_json_databases);
}' > ./$1/model/create_DB.rs

echo '{
    "databases":{
        "sql":{
            "table_test":{
                "id":"INTEGER PRIMARY KEY",
                "name":"TEXT",
                "mail":"TEXT"
            }
        }
    }
}' > ./$1/model/databases.json

echo '
[[bin]]
name = "create_table"
path = "model/create_DB.rs"

[[bin]]
name = "main"
path = "src/main.rs"

[[bin]]
name = "create_json"
path = "src/test/anarisis_code_to_json.rs"

[[bin]]
name = "test"
path = "src/test/test.rs"' >> ./$1/Cargo.toml

echo 'server:
	cargo run --bin main --release

create_db:
	cargo run --bin create_table --release

test:
	cargo run --bin create_json --release

sample:
	cargo run --bin test --release' > ./$1/Makefile

cp ./documents/new_get_page.sh ./$1/
cp ./documents/new_post_page.sh ./$1/
cd $1
cargo add actix-web
cargo add actix-rt
cargo add thiserror
cargo add askama
cargo add rusqlite
cargo add serde
cargo add serde_json
cargo build