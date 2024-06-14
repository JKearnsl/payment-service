use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domain::models::money::Money;
use crate::domain::models::payment_method::PaymentMethod;
use crate::domain::models::payment_state::PaymentState;
use crate::domain::models::user::UserId;

pub type PaymentId = Uuid;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Payment {
    pub id: PaymentId,
    pub state: PaymentState,
    pub method: PaymentMethod,
    pub amount: Money,
    pub seller_id: UserId,
    
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Payment {
    pub fn new(
        method: PaymentMethod,
        amount: Money,
        seller_id: UserId,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            state: PaymentState::Pending,
            method,
            amount,
            seller_id,
            created_at: Utc::now(),
            updated_at: None,
        }
    }

    pub fn update(
        &self,
        state: PaymentState,
    ) -> Self {
        Self {
            state,
            updated_at: Some(Utc::now()),
            ..self.clone()
        }
    }

}
