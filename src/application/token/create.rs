use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::hasher::Hasher;
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::token_gateway::TokenGateway;
use crate::domain::models::token::{Token, TokenKey};
use crate::domain::services::validator::ValidatorService;

pub struct CreateToken<'a> {
    pub token_gateway: &'a dyn TokenGateway,
    pub token_hasher: &'a dyn Hasher,
    pub validator: &'a ValidatorService,
    pub id_provider: Box<dyn IdProvider>,
}

impl Interactor<(), TokenKey> for CreateToken<'_> {
    async fn execute(&self, data: ()) -> Result<TokenKey, ApplicationError> {
        
        if !self.id_provider.is_auth() {
            return Err(ApplicationError::Unauthorized(
                ErrorContent::Message("Unauthorized".to_string()),
            ));
        }
        
        let token_key = Token::generate_token_key();
        let hashed_token = self.token_hasher.hash(&token_key).await;
        
        let model = Token::new(
            self.id_provider.user_id().unwrap().clone(),
            hashed_token,
        );

        self.token_gateway.save_token(&model).await;

        Ok(token_key)
    }
}
