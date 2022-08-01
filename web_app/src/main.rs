use actix_web::{App, HttpServer};
use actix_files::Files;
use web_app::routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("     access:http://127.0.0.1:8000/");
    HttpServer::new(|| App::new()
        .service(Files::new("/css", "./css").show_files_listing())  
        .configure(routes::routes)
        )
        .bind("127.0.0.1:8000")?
        .run()
        .await
}