use actix_web::{get, HttpRequest, HttpResponse, post, Result, web};
use uuid::Uuid;

use crate::application::common::exceptions::ApplicationError;
use crate::application::common::interactor::Interactor;
use crate::application::common::token_processor::TokenProcessor;
use crate::application::user::create::CreateUserDTO;
use crate::application::user::get_by_id::GetUserByIdDTO;
use crate::presentation::id_provider::get_id_provider;
use crate::presentation::interactor_factory::InteractorFactory;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(users_by_id)
            .service(user_self)
            .service(create_user)
    );
}

#[get("{id}")]
async fn users_by_id(
    id: web::Path<Uuid>,
    ioc: web::Data<dyn InteractorFactory>,
    token_processor: web::Data<dyn TokenProcessor>,
    req: HttpRequest
) -> Result<HttpResponse, ApplicationError> {
    
    let id_provider = get_id_provider(&req, &token_processor);

    let data = ioc.get_user_by_id(id_provider).execute(GetUserByIdDTO {
        id: id.into_inner()
    }).await?;
    
    Ok(HttpResponse::Ok().json(data))
}

#[get("/self")]
async fn user_self(
    ioc: web::Data<dyn InteractorFactory>,
    token_processor: web::Data<dyn TokenProcessor>,
    req: HttpRequest
) -> Result<HttpResponse, ApplicationError> {
    let id_provider = get_id_provider(&req, &token_processor);
    let data = ioc.get_user_self(id_provider).execute(()).await?;
    Ok(HttpResponse::Ok().json(data))
}

#[post("")]
async fn create_user(
    data: web::Json<CreateUserDTO>,
    ioc: web::Data<dyn InteractorFactory>,
    token_processor: web::Data<dyn TokenProcessor>,
    req: HttpRequest
) -> Result<HttpResponse, ApplicationError> {
    let id_provider = get_id_provider(&req, &token_processor);
    let data = ioc.create_user(id_provider).execute(data.into_inner()).await?;
    Ok(HttpResponse::Ok().json(data))
}
