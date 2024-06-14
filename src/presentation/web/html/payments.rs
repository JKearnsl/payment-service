use actix_web::{get, HttpResponse, web};
use serde::Deserialize;
use crate::application::common::exceptions::ApplicationError;
use crate::application::common::interactor::Interactor;
use crate::domain::models::payment::PaymentId;
use crate::domain::models::payment_method::PaymentMethod;
use crate::domain::models::payment_state::PaymentState;
use crate::presentation::interactor_factory::InteractorFactory;


pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/payments")
            .service(index)
            .service(process_stub)
            .service(get_by_id)
    );
}


#[get("")]
async fn index(
    tera: web::Data<tera::Tera>
) -> Result<HttpResponse, ApplicationError> {
    Ok(HttpResponse::Ok().body(
        tera.render("payments.html", &tera::Context::new()).unwrap()
    ))
}

#[derive(Deserialize)]
struct ProcessQuery {
    id: PaymentId
}

#[get("process")]
async fn process_stub(
    tera: web::Data<tera::Tera>,
    id: web::Query<ProcessQuery>,
    ioc: web::Data<dyn InteractorFactory>,
    data: web::Query<StoreMeta>
) -> Result<HttpResponse, ApplicationError> {
    let id = id.into_inner().id;
    
    let payment = ioc.get_payment().execute(id).await?;
    if payment.state == PaymentState::Pending {
        ioc.process_payment().execute(id.clone()).await?;
    }
    let mut context = tera::Context::new();
    context.insert("order", &payment.id);
    context.insert("amount", &payment.amount);

    let store =  data.store.clone().unwrap_or("Hidden Store".to_string());

    context.insert("store", &store);

    Ok(HttpResponse::Ok().body(
        tera.render("payment_success.html", &context).unwrap()
    ))
}

#[derive(Deserialize)]
struct StoreMeta {
    store: Option<String>
}


#[get("{id}")]
async fn get_by_id(
    tera: web::Data<tera::Tera>,
    id: web::Path<PaymentId>,
    data: web::Query<StoreMeta>,
    ioc: web::Data<dyn InteractorFactory>,
) -> Result<HttpResponse, ApplicationError> {
    let payment = ioc.get_payment().execute(id.into_inner()).await?;
    let mut context = tera::Context::new();
    context.insert("order", &payment.id);
    context.insert("amount", &payment.amount);

    let store =  data.store.clone().unwrap_or("Hidden Store".to_string());

    context.insert("store", &store);

    Ok(HttpResponse::Ok().body(match payment.state {
        PaymentState::Pending => match payment.method {
            PaymentMethod::Card => tera.render("payment_card.html", &context).unwrap(),
            PaymentMethod::QrCode => tera.render("payment_qr.html", &context).unwrap()
        },
        _ => tera.render("payment_success.html", &context).unwrap()
    }))
}
