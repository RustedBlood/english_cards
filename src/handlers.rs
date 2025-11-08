use actix_web::{
    HttpResponse, Result,
    web::{self, Path},
};
use dashmap::DashMap;
use tera::{Context, Tera};

use crate::models::{self, Word, WordsDash};

//Отвечает за эндпоинт
// /
// возвращает text/html для главной страницы
pub async fn index(tera: web::Data<Tera>) -> Result<HttpResponse> {
    let context = Context::new();

    render_template(&tera, "index.html", context).await
}

//Отвечает за эндпоинт
// /grade
// возвращает text/html для карточек
pub async fn grade(tera: web::Data<Tera>) -> Result<HttpResponse> {
    let context = Context::new();

    render_template(&tera, "cards.html", context).await
}

//Рендерит Tera шаблоны и отправляет text/html
async fn render_template(tera: &Tera, index_name: &str, ctx: Context) -> Result<HttpResponse> {
    let rendered = tera.render(index_name, &ctx).map_err(|e| {
        eprintln!("Ошибка рендера шаблона: {}", e);
        actix_web::error::ErrorInternalServerError("Ошибка шаблона")
    })?;

    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

//API
pub async fn get_words(
    cashe: web::Data<WordsDash>,
    grade_id: Path<String>,
) -> Result<web::Json<models::JsonWords>> {
    let grade = grade_id.into_inner();

    let words = cashe.words_dash.get(&grade).unwrap().clone();

    let new_json = models::JsonWords { grade, words };

    Ok(web::Json(new_json))
}
