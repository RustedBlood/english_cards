use actix_web::{HttpResponse, Result, web};
use tera::{Context, Tera};

pub async fn index(tera: web::Data<Tera>) -> Result<HttpResponse> {
    let context = Context::new();

    render_template(&tera, "index.html", context).await
}

pub async fn grade(tera: web::Data<Tera>, group_id: web::Path<String>) -> Result<HttpResponse> {
    let context = Context::new();

    println!("{}", group_id.into_inner());

    //let index_name = format!("{}.html", group_id.into_inner());

    render_template(&tera, "cards.html", context).await
}

async fn render_template(tera: &Tera, index_name: &str, ctx: Context) -> Result<HttpResponse> {
    let rendered = tera.render(index_name, &ctx).map_err(|e| {
        eprintln!("Ошибка рендера шаблона: {}", e);
        actix_web::error::ErrorInternalServerError("Ошибка шаблона")
    })?;

    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}
