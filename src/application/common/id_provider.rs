use crate::domain::models::user::UserId;

pub trait IdProvider {
    fn user_id(&self) -> Option<&UserId>;
    fn is_auth(&self) -> &bool;
}
