use crate::domain::models::session_token_hash::SessionTokenHash;
use crate::domain::models::user::UserId;


pub trait TokenProcessor {
    fn validate_token(&self, token: &SessionTokenHash) -> Option<&UserId>;
    fn set_token(&mut self, token: SessionTokenHash, user_id: UserId);
}
