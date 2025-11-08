use actix_files::Files;
use actix_web::{App, HttpResponse, HttpServer, Result, web};
use tera::{Context, Tera};

mod csv_loader;
mod handlers;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //обработка .html файлов по templates/ и их загрузка
    let tera = Tera::new("templates/*").unwrap();
    let tera_data = web::Data::new(tera);

    //Запуск и чтение csv ридера
    let loaded_words = models::WordsDash::new();
    let loaded_words_data = web::Data::new(loaded_words);
    //Запуск http сервера
    HttpServer::new(move || {
        App::new()
            .app_data(tera_data.clone()) // клонирование Arc
            .app_data(loaded_words_data.clone()) //arc указатель
            .route("/", web::get().to(handlers::index)) //первичная страница
            .route("/{group_id}", web::get().to(handlers::grade)) // Загрузка .html
            .route("/api/grade/{grade}", web::get().to(handlers::get_words))
            .service(Files::new("/static", "./static")) // Сервис статик файлов
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
