use async_trait::async_trait;

use crate::domain::models::token::{TokenHash, TokenId};

#[async_trait]
pub trait TokenReader {
    async fn get_token(&self, token_id: &TokenId) -> Option<Token>;
    async fn get_token_by_hash(
        &self, 
        token_hash: &TokenHash
    ) -> Option<Token>;
    async fn get_user_tokens(&self, user_id: &UserId) -> Vec<Token>;
}

#[async_trait]
pub trait TokenWriter {
    async fn save_token(&self, data: &Token);
}

#[async_trait]
pub trait TokenRemover {
    async fn remove_token(&self, token_id: &TokenId);
}


pub trait TokenGateway: TokenReader + TokenWriter + TokenRemover {}