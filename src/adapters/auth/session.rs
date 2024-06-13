use std::sync::Arc;
use uuid::Uuid;

use crate::application::common::id_provider::IdProvider;
use crate::application::common::session_processor::{SessionProcessor};
use crate::domain::models::session_token_hash::SessionTokenHash;

pub struct IdSessionProvider {
    user_id: Option<Uuid>,
    is_auth: bool
}


impl IdSessionProvider {
    pub async fn new(
        session_token: Option<SessionTokenHash>,
        session_processor: &dyn SessionProcessor
    ) -> Self {
        match session_token {
            Some(token) => {
                match session_processor.verify_token(&token).await {
                    Some(user_id) => {
                        Self {
                            user_id: Some(user_id.clone()),
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

impl IdProvider for IdSessionProvider {

    fn user_id(&self) -> Option<&Uuid> {
        self.user_id.as_ref()
    }

    fn is_auth(&self) -> &bool {
        &self.is_auth
    }
}
