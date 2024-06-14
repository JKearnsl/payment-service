use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::interactor::Interactor;
use crate::application::common::payment_gateway::PaymentGateway;
use crate::application::common::user_gateway::UserGateway;
use crate::domain::models::money::Money;
use crate::domain::models::payment::PaymentId;
use crate::domain::models::payment_method::PaymentMethod;
use crate::domain::models::payment_state::PaymentState;
use crate::domain::models::user::UserId;

#[derive(Debug, Serialize)]
pub struct PaymentProcessResultDTO{
    pub id: PaymentId,
    pub state: PaymentState,
    pub method: PaymentMethod,
    pub amount: Money,
    pub seller_id: UserId,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}


pub struct PaymentProcessStub<'a> {
    pub payment_gateway: &'a dyn PaymentGateway,
    pub user_gateway: &'a dyn UserGateway,
}

impl Interactor<PaymentId, PaymentProcessResultDTO> for PaymentProcessStub<'_> {
    async fn execute(&self, data: PaymentId) -> Result<PaymentProcessResultDTO, ApplicationError> {
        
        let mut payment = match self.payment_gateway.get_payment_by_id(&data).await {
            Some(u) => u,
            None => return Err(
                ApplicationError::NotFound(
                    ErrorContent::Message("Payment not found".to_string())
                )
            ),
        };
        if payment.state != PaymentState::Pending {
            return Err(
                ApplicationError::Forbidden(
                    ErrorContent::Message("Payment already processed".to_string())
                )
            );
        }
        payment = payment.update(PaymentState::Paid);
        self.payment_gateway.save_payment(&payment).await;
        
        let mut user = self.user_gateway.get_user_by_id(&payment.seller_id).await.unwrap();
        user = user.update(user.balance + payment.amount);
        
        self.user_gateway.save_user(&user).await;

        Ok(PaymentProcessResultDTO {
            id: payment.id,
            state: payment.state,
            method: payment.method,
            amount: payment.amount,
            seller_id: payment.seller_id,
            created_at: payment.created_at,
            updated_at: payment.updated_at
        })
    }
}
