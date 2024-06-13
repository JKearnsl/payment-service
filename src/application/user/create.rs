use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::hasher::Hasher;
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::user_gateway::UserGateway;
use crate::domain::models::user::{User, UserId};
use crate::domain::services::validator::ValidatorService;


#[derive(Debug, Deserialize)]
pub struct CreateUserDTO {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct CreateUserResultDTO{
    id: UserId,
    username: String
}

pub struct CreateUser<'a> {
    pub user_gateway: &'a dyn UserGateway,
    pub password_hasher: &'a dyn Hasher,
    pub validator: &'a ValidatorService,
    pub id_provider: Box<dyn IdProvider>,
}

impl Interactor<CreateUserDTO, CreateUserResultDTO> for CreateUser<'_> {
    async fn execute(&self, data: CreateUserDTO) -> Result<CreateUserResultDTO, ApplicationError> {
        
        if *self.id_provider.is_auth() {
            return Err(ApplicationError::Unauthorized(
                ErrorContent::Message("You are already authorized".to_string())
            ));
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
        
        let user_by_username = self.user_gateway.get_user_by_username_not_sensitive(&data.username).await;
        
        if user_by_username.is_some() {
            validator_err_map.insert("username".to_string(), "User with this username already exists".to_string());
        }

        if !validator_err_map.is_empty() {
            return Err(
                ApplicationError::InvalidData(
                    ErrorContent::Map(validator_err_map)
                )
            )
        }
        
        let hashed_password = self.password_hasher.hash(&data.password).await;
        
        let user = User::new(
            data.username,
            hashed_password,
        );

        self.user_gateway.save_user(&user).await;

        Ok(CreateUserResultDTO {
            id: user.id,
            username: user.username,
        })
    }
}
