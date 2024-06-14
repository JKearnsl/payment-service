use actix_web::{get, HttpRequest, HttpResponse, post, Result, web};

use crate::application::common::exceptions::ApplicationError;
use crate::application::common::interactor::Interactor;
use crate::application::common::session_processor::SessionProcessor;
use crate::application::payment::create::CreatePaymentDTO;
use crate::domain::models::payment::PaymentId;
use crate::presentation::id_session_provider::get_id_session_provider;
use crate::presentation::interactor_factory::InteractorFactory;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/payments")
            .service(create_payment)
            .service(get_payment)
            .service(get_payments)
    );
}

#[post("")]
async fn create_payment(
    ioc: web::Data<dyn InteractorFactory>,
    data: web::Json<CreatePaymentDTO>
) -> Result<HttpResponse, ApplicationError> {
    let result = ioc.create_payment().execute(data.into_inner()).await?;
    Ok(HttpResponse::Ok().json(result))
}

#[get("{id}")]
async fn get_payment(
    ioc: web::Data<dyn InteractorFactory>,
    payment_id: web::Path<PaymentId>,
) -> Result<HttpResponse, ApplicationError> {
    let payment = ioc.get_payment().execute(payment_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(payment))
}

#[get("")]
async fn get_payments(
    ioc: web::Data<dyn InteractorFactory>,
    session_processor: web::Data<dyn SessionProcessor>,
    req: HttpRequest
) -> Result<HttpResponse, ApplicationError> {
    
    let id_provider = get_id_session_provider(
        &req,
        &session_processor
    ).await;
    
    let payments = ioc.get_payments(id_provider).execute(()).await?;
    Ok(HttpResponse::Ok().json(payments))
}