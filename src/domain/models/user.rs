use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domain::models::money::Money;

pub type UserId = Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub username: String,
    pub hashed_password: String,
    pub balance: Money
}

impl User {
    pub fn new(
        username: String,
        hashed_password: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            username,
            hashed_password,
            balance: Money::new(0, 2)
        }
    }
    
    pub fn update(
        &self,
        balance: Money,
    ) -> Self {
        Self {
            balance,
            ..self.clone()
        }
    }
}
