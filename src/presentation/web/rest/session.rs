use actix_web::{delete, HttpRequest, HttpResponse, post, Result, web};
use actix_web::cookie::Cookie;

use crate::application::common::exceptions::ApplicationError;
use crate::application::common::interactor::Interactor;
use crate::application::common::token_processor::TokenProcessor;
use crate::application::session::create::CreateSessionDTO;
use crate::presentation::id_provider::get_id_provider;
use crate::presentation::interactor_factory::InteractorFactory;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/sessions")
            .service(create_session)
            .service(delete_self_session)
    );
}

#[post("")]
async fn create_session(
    data: web::Json<CreateSessionDTO>,
    ioc: web::Data<dyn InteractorFactory>,
    token_processor: web::Data<dyn TokenProcessor>,
    req: HttpRequest
) -> Result<HttpResponse, ApplicationError> {
    let id_provider = get_id_provider(&req, &token_processor);
    let (data, session_token) = ioc.create_session(id_provider).execute(
        data.into_inner()
    ).await?;
    
    let mut response = HttpResponse::Ok().json(data);
    response.add_cookie(
        &Cookie::build("session_token", session_token.to_string())
            .path("/")
            .http_only(true)
            .finish()
    ).unwrap();
    
    Ok(response)
}

#[delete("self")]
async fn delete_self_session(
    ioc: web::Data<dyn InteractorFactory>,
    token_processor: web::Data<dyn TokenProcessor>,
    req: HttpRequest
) -> Result<HttpResponse, ApplicationError> {
    let id_provider = get_id_provider(&req, &token_processor);
    ioc.delete_self_session(id_provider).execute(()).await?;
    Ok(HttpResponse::Ok().finish())
}
