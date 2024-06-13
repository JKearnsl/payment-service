use async_trait::async_trait;
use crate::domain::models::session_token::SessionToken;
use crate::domain::models::user::UserId;


#[async_trait]
pub trait SessionProcessor {
    async fn verify_token(&self, token: &SessionToken) -> Option<UserId>;
    async fn set_token(&self, user_id: UserId) -> SessionToken;
    async fn remove_token(&self, token: &SessionToken);
}
