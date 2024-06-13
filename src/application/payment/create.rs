use std::collections::HashMap;
use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};
use crate::adapters::auth::token::IdTokenProvider;

use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::hasher::Hasher;
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::payment_gateway::PaymentGateway;
use crate::application::common::token_gateway::TokenReader;
use crate::domain::models::money::Money;
use crate::domain::models::payment_method::PaymentMethod;
use crate::domain::models::payment_state::PaymentState;
use crate::domain::models::payment::{Payment, PaymentId};
use crate::domain::models::token::TokenKey;
use crate::domain::models::user::UserId;
use crate::domain::services::validator::ValidatorService;


#[derive(Debug, Deserialize)]
pub struct CreatePaymentDTO {
    pub method: PaymentMethod,
    pub amount: Money,
    pub token_key: Option<TokenKey>
}

#[derive(Debug, Serialize)]
pub struct CreatePaymentResultDTO{
    pub id: PaymentId,
    pub state: PaymentState,
    pub method: PaymentMethod,
    pub amount: Money,
    pub seller_id: UserId,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

pub struct CreatePayment<'a> {
    pub payment_gateway: &'a dyn PaymentGateway,
    pub token_reader: &'a dyn TokenReader,
    pub token_hasher: &'a dyn Hasher,
    pub password_hasher: &'a dyn Hasher,
    pub validator: &'a ValidatorService,
}

impl Interactor<CreatePaymentDTO, CreatePaymentResultDTO> for CreatePayment<'_> {
    async fn execute(&self, data: CreatePaymentDTO) -> Result<CreatePaymentResultDTO, ApplicationError> {
        
        let id_provider = Box::new(
            IdTokenProvider::new(
                data.token_key,
                self.token_reader,
                self.token_hasher
            ).await
        );
        
        if !id_provider.is_auth() {
            return Err(ApplicationError::Unauthorized(
                ErrorContent::Message("Необходимо авторизоваться".to_string())
            ));
        }

        let mut validator_err_map: HashMap<String, String> = HashMap::new();
        self.validator.validate_payment_amount(&data.amount).unwrap_or_else(|e| {
            validator_err_map.insert("amount".to_string(), e.to_string());
        });

        if !validator_err_map.is_empty() {
            return Err(
                ApplicationError::InvalidData(
                    ErrorContent::Map(validator_err_map)
                )
            )
        }
        
        let payment = Payment::new(
            data.method,
            data.amount,
            id_provider.user_id().unwrap().clone()
        );

        self.payment_gateway.save_payment(&payment).await;

        Ok(CreatePaymentResultDTO {
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
