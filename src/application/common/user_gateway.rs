use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::models::user::User as UserDomain;

#[async_trait]
pub trait UserReader {
    async fn get_user_by_id(&self, user_id: &Uuid) -> Option<UserDomain>;
    async fn get_user_by_username_not_sensitive(&self, username: &String) -> Option<UserDomain>;
}

#[async_trait]
pub trait UserWriter {
    async fn save_user(&self, data: &UserDomain);
}

pub trait UserGateway: UserReader + UserWriter {}