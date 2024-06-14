use serde::Deserialize;

use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::token_gateway::TokenGateway;
use crate::domain::models::token::TokenId;

#[derive(Debug, Deserialize)]
pub struct DeleteTokenDTO{
    id: TokenId,
}


pub struct DeleteToken<'a> {
    pub token_gateway: &'a dyn TokenGateway,
    pub id_provider: Box<dyn IdProvider>,
}

impl Interactor<DeleteTokenDTO, ()> for DeleteToken<'_> {
    async fn execute(&self, data: DeleteTokenDTO) -> Result<(), ApplicationError> {
        
        if !self.id_provider.is_auth() {
            return Err(
                ApplicationError::Unauthorized(
                    ErrorContent::Message("Unauthorized".to_string())
                )
            );
        }
        
        let token = match self.token_gateway.get_token(&data.id).await {
            Some(token) => token,
            None => {
                return Err(
                    ApplicationError::NotFound(
                        ErrorContent::Message("Token not found".to_string())
                    )
                );
            }
        };

        if token.user_id != *self.id_provider.user_id().unwrap() {
            return Err(
                ApplicationError::Forbidden(
                    ErrorContent::Message("Forbidden".to_string())
                )
            );
        }
        
        self.token_gateway.remove_token(&data.id).await;

        Ok(())
    }
}
