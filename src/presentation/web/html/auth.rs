use actix_web::{get, HttpResponse, web};
use crate::application::common::exceptions::ApplicationError;


pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(login)
            .service(register)
    );
}


#[get("login")]
async fn login(
    tera: web::Data<tera::Tera>
) -> Result<HttpResponse, ApplicationError> {
    Ok(HttpResponse::Ok().body(
        tera.render("login.html", &tera::Context::new()).unwrap()
    ))
}


#[get("register")]
async fn register(
    tera: web::Data<tera::Tera>
) -> Result<HttpResponse, ApplicationError> {
    Ok(HttpResponse::Ok().body(
        tera.render("register.html", &tera::Context::new()).unwrap()
    ))
}