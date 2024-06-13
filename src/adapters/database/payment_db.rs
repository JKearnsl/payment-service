use async_trait::async_trait;

use sea_orm::{DbConn, EntityTrait, QueryFilter};
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::Expr;

use crate::adapters::database::models::payment;
use crate::adapters::database::models::sea_orm_active_enums::{
    PaymentState,
    PaymentMethod
};
use crate::application::common::payment_gateway::{
    PaymentGateway as PaymentGatewayTrait, 
    PaymentReader, 
    PaymentWriter
};
use crate::domain::models::payment::{
    Payment,
    PaymentId, 
};
use crate::domain::models::payment_method::PaymentMethod as DomainPaymentMethod;
use crate::domain::models::payment_state::PaymentState as DomainPaymentState;
use crate::domain::models::user::UserId;

pub struct PaymentGateway {
    db: Box<DbConn>,
}

impl PaymentGateway {
    pub fn new(
        db: Box<DbConn>
    ) -> Self {
        PaymentGateway {
            db
        }
    }
}

#[async_trait]
impl PaymentReader for PaymentGateway {
    async fn get_payment_by_id(&self, payment_id: &PaymentId) -> Option<Payment> {
        match payment::Entity::find_by_id(payment_id.clone())
            .one(&*self.db)
            .await.unwrap() {
            Some(model) => Some(map_session_model_to_domain(model)),
            None => None
        }
    }
    
    async fn get_payments_by_user_id(&self, user_id: &UserId) -> Vec<Payment> {
        let payments: Vec<payment::Model> = payment::Entity::find()
            .filter(Expr::col(payment::Column::SellerId).eq(*user_id))
            .all(&*self.db)
            .await
            .unwrap();

        payments.iter().map(
            |model| map_session_model_to_domain(model.clone())
        ).collect()
    }
}

#[async_trait]
impl PaymentWriter for PaymentGateway {
    async fn save_payment(&self, data: &Payment) {
        let model = payment::ActiveModel {
            id: Set(data.id),
            state: Set({
                match data.state {
                    DomainPaymentState::Pending => PaymentState::Pending,
                    DomainPaymentState::Paid => PaymentState::Paid,
                    DomainPaymentState::Rejected => PaymentState::Rejected,
                }
            }),
            method: Set({
                match data.method {
                    DomainPaymentMethod::Card => PaymentMethod::Card,
                    DomainPaymentMethod::QrCode => PaymentMethod::QrCode,
                }
            }),
            amount: Set(data.amount),
            seller_id: Set(data.seller_id),
            created_at: Set(data.created_at),
            updated_at: Set(data.updated_at)
        };
        
        match payment::Entity::find_by_id(data.id).one(&*self.db).await.unwrap() {
            Some(_) => {
                payment::Entity::update(model).exec(&*self.db).await.unwrap();
            },
            None => {
                payment::Entity::insert(model).exec(&*self.db).await.unwrap();
            }
        }
    }
}

impl PaymentGatewayTrait for PaymentGateway {}

fn map_session_model_to_domain(model: payment::Model) -> Payment {
    Payment {
        id: model.id,
        state: {
            match model.state {
                PaymentState::Pending => DomainPaymentState::Pending,
                PaymentState::Paid => DomainPaymentState::Paid,
                PaymentState::Rejected => DomainPaymentState::Rejected,
            }
        },
        method: {
            match model.method {
                PaymentMethod::Card => DomainPaymentMethod::Card,
                PaymentMethod::QrCode => DomainPaymentMethod::QrCode,
            }
        },
        seller_id: model.seller_id,
        amount: model.amount,
        created_at: model.created_at,
        updated_at: model.updated_at,
    }
}
