setting_directory(){
    cargo new $1
    # npm i fabric
    mkdir ./$1/src/controllers
    mkdir ./$1/templates
    mkdir ./$1/templates/model
    mkdir ./$1/templates/view
    mkdir ./$1/DB_Model
    mkdir ./$1/DB_Model/db
    mkdir ./$1/css
    mkdir ./$1/css/model
    mkdir ./$1/css/view
    touch ./$1/css/model/style.css
    touch ./$1/css/view/style.css
    mkdir ./$1/src/test
    mkdir ./$1/src/test/data_json
    mkdir ./$1/src/test/test_Model
    touch ./$1/src/test/data_json/model_file.json
    touch ./$1/src/test/data_json/input_file.json
}

create_html() {
    echo '<!DOCTYPE html>
    <html>
    <head>
        <meta charset="utf-8" />
        <link rel="stylesheet" href="../../css/view/style.css">
        <title>App</title>
    </head>
    <body>
        <div>
            <h1>{{text}}</h1>
        </div>
    </body>
    </html>' > ./$1/templates/view/index.html
}

create_server(){
    echo 'use actix_web::{App, HttpServer};
    use actix_files as fs;
    use '$1'::routes;

    #[actix_web::main]
    async fn main() -> std::io::Result<()> {
        HttpServer::new(|| App::new()
            .configure(routes::routes)
            .service(fs::Files::new("/css", "./css").show_files_listing()))
            .bind("localhost:8000")?
            .run()
            .await
    }' > ./$1/src/main.rs
}

create_controller(){
    echo 'use actix_web::{ HttpResponse, Responder, ResponseError};
    use askama::Template;
    use thiserror::Error;
    // index.htmlのテンプレート定義
    #[derive(Template)]
    #[template(path="view/index.html")]
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
}
 
create_controller_mod(){
    echo 'pub mod index;' > ./$1/src/controllers/mod.rs
}

create_lib(){
    echo 'pub mod routes;
    pub mod controllers;' > ./$1/src/lib.rs
}

create_routes(){
    echo 'use actix_web::web;
    use crate::controllers::*;
    pub fn routes(cfg: &mut web::ServiceConfig) {
        cfg.route("/", web::get().to(index::index));
    }' > ./$1/src/routes.rs
}

create_DB_Rust(){
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
    }' > ./$1/DB_Model/create_DB.rs
}

create_DB_JSON(){
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
    }' > ./$1/DB_Model/databases.json
}

create_Cargo_toml(){
    echo '[[bin]]
    name = "create_table"
    path = "DB_Model/create_DB.rs"

    [[bin]]
    name = "main"
    path = "src/main.rs"

    [[bin]]
    name = "create_json"
    path = "src/test/anarisis_code_to_json.rs"

    [[bin]]
    name = "test"
    path = "src/test/test.rs"

    [[bin]]
    name = "test_Model"
    path = "src/test/test_Model/main.rs"' >> ./$1/Cargo.toml

    echo 'server:
        cargo run --bin main --release

    create_db:
        cargo run --bin create_table --release

    test:
        cargo run --bin create_json --release

    sample:
        cargo run --bin test --release' > ./$1/Makefile
}

create_new_get_page(){
    echo r""
}

setting_develop(){
    cargo add actix-web
    cargo add actix-files
    cargo add actix-rt
    cargo add thiserror
    cargo add askama
    cargo add rusqlite
    cargo add serde
    cargo add serde_json
    cargo add scraper
    cargo add glob
    cargo add html5ever
    cargo add serde_derive
    cargo build
    cd src/test
    npx create-react-app test_Model
}

setting_directory $1
create_html $1
create_server $1
create_controller $1
create_routes $1
create_DB_Rust $1
create_DB_JSON $1
create_Cargo_toml $1