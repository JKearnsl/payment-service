use chrono::{DateTime, Utc};
use serde::Serialize;
use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::interactor::Interactor;
use crate::application::common::payment_gateway::PaymentReader;
use crate::domain::models::money::Money;
use crate::domain::models::payment::{Payment, PaymentId};
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
    pub payment_gateway: &'a dyn PaymentReader,
}

impl Interactor<PaymentId, PaymentProcessResultDTO> for PaymentProcessStub<'_> {
    async fn execute(&self, data: PaymentId) -> Result<PaymentProcessResultDTO, ApplicationError> {
        
        let payment = match self.payment_gateway.get_payment_by_id(&data).await {
            Some(u) => u.update(PaymentState::Paid),
            None => return Err(
                ApplicationError::NotFound(
                    ErrorContent::Message("Платеж не найден".to_string())
                )
            ),
        };

        Ok((PaymentProcessResultDTO {
            id: payment.id,
            state: payment.state,
            method: payment.method,
            amount: payment.amount,
            seller_id: payment.seller_id,
            created_at: payment.created_at,
            updated_at: payment.updated_at
        }))
    }
}
