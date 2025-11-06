use actix_files::Files;
use actix_web::{App, HttpResponse, HttpServer, Result, web};
use tera::{Context, Tera};

mod csv_loader;
mod handlers;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tera = Tera::new("templates/*").unwrap();
    let tera_data = web::Data::new(tera);

    HttpServer::new(move || {
        App::new()
            .app_data(tera_data.clone()) // клонирование Arc
            .route("/", web::get().to(handlers::index))
            .route("/{group_id}", web::get().to(handlers::grade))
            .service(Files::new("/static", "./static"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

//Загрузка файлов .html
