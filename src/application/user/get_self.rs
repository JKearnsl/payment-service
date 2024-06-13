use serde::Serialize;

use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::user_gateway::UserReader;
use crate::domain::models::user::UserId;

#[derive(Debug, Serialize)]
pub struct UserSelfResultDTO{
    id: UserId,
    username: String,
}


pub struct GetUserSelf<'a> {
    pub user_reader: &'a dyn UserReader,
    pub id_provider: Box<dyn IdProvider>,
}

impl Interactor<(), UserSelfResultDTO> for GetUserSelf<'_> {
    async fn execute(&self, data: ()) -> Result<UserSelfResultDTO, ApplicationError> {
        
        if !self.id_provider.is_auth() {
            return Err(
                ApplicationError::Unauthorized(
                    ErrorContent::Message("Необходимо авторизоваться".to_string())
                )
            );
        }
        
        
        let user = self.user_reader.get_user_by_id(
            self.id_provider.user_id().unwrap()
        ).await.unwrap();

        Ok(UserSelfResultDTO {
            id: user.id,
            username: user.username,
        })
    }
}