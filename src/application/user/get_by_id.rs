use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::interactor::Interactor;
use crate::application::common::user_gateway::UserReader;

#[derive(Debug, Deserialize)]
pub struct GetUserByIdDTO {
    pub id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct UserByIdResultDTO{
    id: Uuid,
    username: String,
}


pub struct GetUserById<'a> {
    pub user_reader: &'a dyn UserReader,
}

impl Interactor<GetUserByIdDTO, UserByIdResultDTO> for GetUserById<'_> {
    async fn execute(&self, data: GetUserByIdDTO) -> Result<UserByIdResultDTO, ApplicationError> {
        
        let user = match self.user_reader.get_user_by_id(&data.id).await {
            Some(u) => u,
            None => return Err(
                ApplicationError::NotFound(
                    ErrorContent::Message("User not found".to_string())
                )
            ),
        };

        Ok(UserByIdResultDTO {
            id: user.id,
            username: user.username,
        })
    }
}