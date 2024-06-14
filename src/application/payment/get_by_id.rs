use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::interactor::Interactor;
use crate::application::common::payment_gateway::PaymentReader;
use crate::domain::models::money::Money;
use crate::domain::models::payment::PaymentId;
use crate::domain::models::payment_method::PaymentMethod;
use crate::domain::models::payment_state::PaymentState;
use crate::domain::models::user::UserId;

#[derive(Debug, Serialize)]
pub struct PaymentByIdResultDTO{
    pub id: PaymentId,
    pub state: PaymentState,
    pub method: PaymentMethod,
    pub amount: Money,
    pub seller_id: UserId,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}


pub struct GetPayment<'a> {
    pub payment_reader: &'a dyn PaymentReader,
}

impl Interactor<PaymentId, PaymentByIdResultDTO> for GetPayment<'_> {
    async fn execute(&self, data: PaymentId) -> Result<PaymentByIdResultDTO, ApplicationError> {
        
        let payment = match self.payment_reader.get_payment_by_id(&data).await {
            Some(u) => u,
            None => return Err(
                ApplicationError::NotFound(
                    ErrorContent::Message("Payment not found".to_string())
                )
            ),
        };

        
        Ok(PaymentByIdResultDTO {
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
