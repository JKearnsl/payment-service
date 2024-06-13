use chrono::{DateTime, Utc};
use rand::random;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domain::models::user::UserId;

pub type TokenId = Uuid;

pub type TokenKey = String;

pub type TokenHash = String;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Token {
    pub id: TokenId,
    pub user_id: UserId,
    pub hash: TokenHash,
    pub created_at: DateTime<Utc>,
}

impl Token {
    pub fn new(
        user_id: UserId,
        hash: TokenHash,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            hash,
            created_at: Utc::now(),
        }
    }
    
    pub fn generate_token_key() -> TokenKey {
        (0..32).map(|_| format!("{:02x}", random::<u8>())).collect::<Vec<_>>().join("")
    }
}

