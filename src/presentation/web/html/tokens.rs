use actix_web::{get, HttpResponse, web};
use crate::application::common::exceptions::ApplicationError;


pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tokens")
            .service(index)
    );
}


#[get("")]
async fn index(
    tera: web::Data<tera::Tera>
) -> Result<HttpResponse, ApplicationError> {
    Ok(HttpResponse::Ok().body(
        tera.render("tokens.html", &tera::Context::new()).unwrap()
    ))
}

