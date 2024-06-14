use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::hasher::Hasher;
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::user_gateway::UserReader;
use crate::domain::models::user::UserId;
use crate::domain::services::validator::ValidatorService;

#[derive(Debug, Deserialize)]
pub struct CreateSessionDTO {
    username: String,
    password: String
}

#[derive(Debug, Serialize)]
pub struct CreateSessionResultDTO{
    id: UserId,
    username: String,
}

pub struct CreateSession<'a> {
    pub user_gateway: &'a dyn UserReader,
    pub session_hasher: &'a dyn Hasher,
    pub id_provider: Box<dyn IdProvider>,
    pub password_hasher: &'a dyn Hasher,
    pub validator: &'a ValidatorService
}

impl Interactor<CreateSessionDTO, (CreateSessionResultDTO, UserId)> for CreateSession<'_> {
    async fn execute(
        &self, 
        data: CreateSessionDTO
    ) -> Result<(CreateSessionResultDTO, UserId), ApplicationError> {

        if *self.id_provider.is_auth() {
            return Err(
                ApplicationError::Forbidden(
                    ErrorContent::Message("You are already authorized".to_string())
                )
            )
        }

        let mut validator_err_map: HashMap<String, String> = HashMap::new();
        
        self.validator.validate_username(&data.username).unwrap_or_else(|e| {
            validator_err_map.insert("username".to_string(), e.to_string());
        });

        self.validator.validate_password(&data.password).unwrap_or_else(|e| {
            validator_err_map.insert("password".to_string(), e.to_string());
        });


        if !validator_err_map.is_empty() {
            return Err(
                ApplicationError::InvalidData(
                    ErrorContent::Map(validator_err_map)
                )
            )
        }
        
        let user = match self.user_gateway.get_user_by_username_not_sensitive(&data.username).await {
            Some(user) => user,
            None => return Err(
                ApplicationError::InvalidData(
                    ErrorContent::Message("Invalid username and password pair".to_string())
                )
            )
        };

        match self.password_hasher.verify(
            &data.password,
            &user.hashed_password
        ).await {
            true => true,
            false => return Err(
                ApplicationError::InvalidData(
                    ErrorContent::Message("Invalid username and password pair".to_string())
                )
            )
        };

        Ok((
            CreateSessionResultDTO {
                id: user.id,
                username: user.username,
            },
            user.id
        ))
    }
}
