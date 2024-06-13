use actix_web::{get, HttpRequest, HttpResponse, post, Result, web};
use uuid::Uuid;

use crate::application::common::exceptions::ApplicationError;
use crate::application::common::interactor::Interactor;
use crate::application::common::session_processor::SessionProcessor;
use crate::application::user::create::CreateUserDTO;
use crate::application::user::get_by_id::GetUserByIdDTO;
use crate::presentation::id_session_provider::get_id_session_provider;
use crate::presentation::interactor_factory::InteractorFactory;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(users_by_id)
            .service(user_self)
            .service(create_user)
    );
}

#[get("/self")]
async fn user_self(
    ioc: web::Data<dyn InteractorFactory>,
    session_processor: web::Data<dyn SessionProcessor>,
    req: HttpRequest
) -> Result<HttpResponse, ApplicationError> {
    let id_provider = get_id_session_provider(&req, &session_processor).await;
    let data = ioc.get_user_self(id_provider).execute(()).await?;
    Ok(HttpResponse::Ok().json(data))
}

#[get("")]
async fn users_by_id(
    id: web::Query<Uuid>,
    ioc: web::Data<dyn InteractorFactory>,
) -> Result<HttpResponse, ApplicationError> {
    
    let data = ioc.get_user_by_id().execute(GetUserByIdDTO {
        id: id.into_inner()
    }).await?;
    
    Ok(HttpResponse::Ok().json(data))
}

#[post("")]
async fn create_user(
    data: web::Json<CreateUserDTO>,
    ioc: web::Data<dyn InteractorFactory>,
    session_processor: web::Data<dyn SessionProcessor>,
    req: HttpRequest
) -> Result<HttpResponse, ApplicationError> {
    let id_provider = get_id_session_provider(&req, &session_processor).await;
    let data = ioc.create_user(id_provider).execute(data.into_inner()).await?;
    Ok(HttpResponse::Ok().json(data))
}
