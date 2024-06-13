use actix_web::{get, HttpRequest, HttpResponse, post, web};

use crate::application::common::exceptions::ApplicationError;
use crate::application::common::interactor::Interactor;
use crate::application::common::session_processor::SessionProcessor;
use crate::presentation::id_session_provider::get_id_session_provider;
use crate::presentation::interactor_factory::InteractorFactory;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tokens")
            .service(create_token)
            .service(get_tokens)
    );
}

#[post("")]
async fn create_token(
    ioc: web::Data<dyn InteractorFactory>,
    session_processor: web::Data<dyn SessionProcessor>,
    req: HttpRequest
) -> actix_web::Result<HttpResponse, ApplicationError> {
    let id_provider = get_id_session_provider(&req, &session_processor).await;
    let token_key = ioc.create_token(id_provider).execute(()).await?;
    Ok(HttpResponse::Ok().json(token_key))
}

#[get("")]
async fn get_tokens(
    ioc: web::Data<dyn InteractorFactory>,
    session_processor: web::Data<dyn SessionProcessor>,
    req: HttpRequest
) -> actix_web::Result<HttpResponse, ApplicationError> {
    let id_provider = get_id_session_provider(&req, &session_processor).await;
    let data = ioc.get_tokens(id_provider).execute(()).await?;
    Ok(HttpResponse::Ok().json(data))
}