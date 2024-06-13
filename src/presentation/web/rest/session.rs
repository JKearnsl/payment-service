use std::str::FromStr;
use actix_web::{delete, HttpRequest, HttpResponse, post, Result, web};
use actix_web::cookie::Cookie;

use crate::application::common::exceptions::ApplicationError;
use crate::application::common::interactor::Interactor;
use crate::application::common::session_processor::SessionProcessor;
use crate::application::session::create::CreateSessionDTO;
use crate::domain::models::session_token::SessionToken;
use crate::presentation::id_session_provider::get_id_session_provider;
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
    session_processor: web::Data<dyn SessionProcessor>,
    req: HttpRequest
) -> Result<HttpResponse, ApplicationError> {
    let id_provider = get_id_session_provider(&req, &session_processor).await;
    
    let (data, user_id) = ioc.create_session(id_provider).execute(
        data.into_inner()
    ).await?;
    
    let session_token = session_processor.set_token(
        user_id,
    ).await;
    
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
    session_processor: web::Data<dyn SessionProcessor>,
    req: HttpRequest
) -> Result<HttpResponse, ApplicationError> {
    let id_provider = get_id_session_provider(&req, &session_processor).await;
    let session_token = req.cookie("session_token")
        .map(|cookie| SessionToken::from_str(cookie.value()).unwrap());
    
    let token = ioc.delete_self_session(id_provider).execute(session_token).await?;
    
    session_processor.remove_token(&token).await;
    
    Ok(HttpResponse::Ok().finish())
}
