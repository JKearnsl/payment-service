use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::token_gateway::TokenReader;
use crate::domain::models::token::{TokenHash, TokenId};
use crate::domain::models::user::UserId;

#[derive(Debug, Serialize)]
pub struct TokenListItemDTO{
    id: TokenId,
    user_id: UserId,
    hash: TokenHash,
    created_at: DateTime<Utc>
}


pub type GetTokenListResultDTO = Vec<TokenListItemDTO>;


pub struct GetTokenList<'a> {
    pub token_reader: &'a dyn TokenReader,
    pub id_provider: Box<dyn IdProvider>,
}

impl Interactor<(), GetTokenListResultDTO> for GetTokenList<'_> {
    async fn execute(&self, data: ()) -> Result<GetTokenListResultDTO, ApplicationError> {
        
        if !self.id_provider.is_auth() {
            return Err(
                ApplicationError::Unauthorized(
                    ErrorContent::Message("Unauthorized".to_string())
                )
            );
        }
        
        let tokens = self.token_reader.get_user_tokens(
            self.id_provider.user_id().unwrap()
        ).await;

        Ok(tokens.into_iter().map(|t| TokenListItemDTO {
            id: t.id,
            user_id: t.user_id,
            hash: t.hash,
            created_at: t.created_at
        }).collect())
    }
}
