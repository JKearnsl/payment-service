use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use async_trait::async_trait;
use crate::application::common::session_processor::SessionProcessor;
use crate::domain::models::session_token::SessionToken;
use crate::domain::models::session_token_hash::SessionTokenHash;
use crate::domain::models::user::UserId;

pub struct MemSessionProcessor {
    data: Arc<Mutex<HashMap<SessionTokenHash, UserId>>>
}

impl MemSessionProcessor {
    pub fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(HashMap::new()))
        }
    }
}

#[async_trait]
impl SessionProcessor for MemSessionProcessor {
    async fn verify_token(&self, token: &SessionToken) -> Option<UserId> {
        let data = self.data.lock().unwrap();
        data.get(token).map(|id| id.clone())
    }

    async fn set_token(&self, user_id: UserId) -> SessionToken {
        let token = SessionToken::new();
        
        let mut data = self.data.lock().unwrap();
        data.insert(token.clone(), user_id);
        token
    }

    async fn remove_token(&self, token: &SessionToken) {
        let mut data = self.data.lock().unwrap();
        data.remove(token);
    }
}