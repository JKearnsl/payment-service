use core::option::Option;

use async_trait::async_trait;
use sea_orm::{ColumnTrait, DbConn, EntityTrait, QueryFilter};
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::Expr;
use uuid::Uuid;

use crate::adapters::database::models::token;
use crate::application::common::token_gateway::{
    TokenGateway as TokenGatewayTrait,
    TokenReader,
    TokenRemover,
    TokenWriter
};
use crate::domain::models::token::{Token as TokenDomain, TokenHash, TokenId};
use crate::domain::models::user::UserId;

pub struct TokenGateway{
    db: Box<DbConn>,
}

impl TokenGateway {
    pub fn new(db: Box<DbConn>) -> Self {
        TokenGateway {
            db,
        }
    }
}

#[async_trait]
impl TokenReader for TokenGateway {
    async fn get_token(&self, token_id: &Uuid) -> Option<TokenDomain> {
        match token::Entity::find_by_id(token_id.clone()).one(&*self.db).await.unwrap() {
            Some(token) => {
                Option::from(map_token_model_to_domain(token))
            }
            None => None
        }
    }

    async fn get_token_by_hash(&self, token_hash: &TokenHash) -> Option<TokenDomain> {
        match token::Entity::find()
            .filter(token::Column::Hash.eq(token_hash.clone()))
            .one(&*self.db).await.unwrap() {
            Some(token) => {
                Option::from(map_token_model_to_domain(token))
            }
            None => None
        }
    }
    
    async fn get_user_tokens(&self, user_id: &UserId) -> Vec<TokenDomain> {
        let tokens: Vec<token::Model> = token::Entity::find()
            .filter(Expr::col(token::Column::UserId).eq(*user_id))
            .all(&*self.db)
            .await
            .unwrap();
        tokens.iter().map(|token| map_token_model_to_domain(token.clone())).collect()
    }
}

#[async_trait]
impl TokenWriter for TokenGateway {
    async fn save_token(&self, data: &TokenDomain) {
        let token_model = token::ActiveModel {
            id: Set(data.id),
            user_id: Set(data.user_id),
            hash: Set(data.hash.clone()),
            created_at: Set(data.created_at),
        };

        match token::Entity::find_by_id(data.id).one(&*self.db).await.unwrap() {
            Some(_) => {
                token::Entity::update(token_model).exec(&*self.db).await.unwrap();
            }
            None => {
                token::Entity::insert(token_model).exec(&*self.db).await.unwrap();
            }
        }
    }
}

#[async_trait]
impl TokenRemover for TokenGateway {
    async fn remove_token(&self, token_id: &TokenId) {
        token::Entity::delete_by_id(token_id.clone())
            .exec(&*self.db)
            .await
            .unwrap();
        
    }
}

fn map_token_model_to_domain(token: token::Model) -> TokenDomain {
    TokenDomain {
        id: token.id,
        hash: token.hash,
        user_id: token.user_id,
        created_at: token.created_at,
    }
}


impl TokenGatewayTrait for TokenGateway {}
