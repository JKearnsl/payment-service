use uuid::Uuid;

use crate::application::common::hasher::Hasher;
use crate::application::common::id_provider::IdProvider;
use crate::application::common::token_gateway::TokenReader;
use crate::domain::models::token::{TokenHash, TokenKey};

pub struct IdTokenProvider {
    user_id: Option<Uuid>,
    is_auth: bool
}


impl IdTokenProvider {
    pub async fn new(
        session_token: Option<TokenKey>,
        token_gateway: &dyn TokenReader,
        session_token_hasher: &dyn Hasher
    ) -> Self {
        match session_token {
            Some(token) => {
                let token_hash: TokenHash = TokenHash::from(session_token_hasher.hash(token.as_str()).await);
                match token_gateway.get_token_by_hash(&token_hash).await {
                    Some(token_model) => {
                        Self {
                            user_id: Some(token_model.user_id.clone()),
                            is_auth: true
                        }
                    },
                    None => {
                        Self {
                            user_id: None,
                            is_auth: false
                        }
                    }
                }
            },
            None => {
                Self {
                    user_id: None,
                    is_auth: false
                }
            }
        }
    }
}

impl IdProvider for IdTokenProvider {

    fn user_id(&self) -> Option<&Uuid> {
        self.user_id.as_ref()
    }

    fn is_auth(&self) -> &bool {
        &self.is_auth
    }
}
