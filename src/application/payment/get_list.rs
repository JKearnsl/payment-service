use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::payment_gateway::PaymentReader;
use crate::domain::models::money::Money;
use crate::domain::models::payment::PaymentId;
use crate::domain::models::payment_method::PaymentMethod;
use crate::domain::models::payment_state::PaymentState;
use crate::domain::models::user::UserId;

#[derive(Debug, Serialize)]
pub struct PaymentListItemDTO{
    id: PaymentId,
    state: PaymentState,
    method: PaymentMethod,
    amount: Money,
    seller_id: UserId,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}


pub type GetPaymentListResultDTO = Vec<PaymentListItemDTO>;


pub struct GetPaymentList<'a> {
    pub payment_reader: &'a dyn PaymentReader,
    pub id_provider: Box<dyn IdProvider>,
}

impl Interactor<(), GetPaymentListResultDTO> for GetPaymentList<'_> {
    async fn execute(&self, data: ()) -> Result<GetPaymentListResultDTO, ApplicationError> {

        if !self.id_provider.is_auth() {
            return Err(
                ApplicationError::Unauthorized(
                    ErrorContent::Message("Unauthorized".to_string())
                )
            );
        }

        let payments = self.payment_reader.get_payments_by_user_id(
            self.id_provider.user_id().unwrap()
        ).await;

        Ok(payments.into_iter().map(|t| PaymentListItemDTO {
            id: t.id,
            state: t.state,
            method: t.method,
            amount: t.amount,
            seller_id: t.seller_id,
            created_at: t.created_at,
            updated_at: t.updated_at
        }).collect())
    }
}
