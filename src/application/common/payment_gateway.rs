use async_trait::async_trait;

use crate::domain::models::payment::{Payment, PaymentId};
use crate::domain::models::user::UserId;

#[async_trait]
pub trait PaymentReader {
    async fn get_payment_by_id(&self, payment_id: &PaymentId) -> Option<Payment>;
    async fn get_payments_by_user_id(&self, user_id: &UserId) -> Vec<Payment>;
}

#[async_trait]
pub trait PaymentWriter {
    async fn save_payment(&self, data: &Payment);
}


pub trait PaymentGateway: PaymentReader + PaymentWriter {}