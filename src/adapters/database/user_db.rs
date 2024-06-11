use core::option::Option;

use async_trait::async_trait;
use sea_orm::{DbConn, EntityTrait, QueryFilter};
use sea_orm::ActiveValue::Set;
use sea_orm::sea_query::Expr;
use sea_orm::sea_query::extension::postgres::PgExpr;

use crate::adapters::database::models::users;
use crate::application::common::user_gateway::{
    UserGateway as UserGatewayTrait,
    UserReader,
    UserWriter
};
use crate::domain::models::user::{User as UserDomain, UserId};

pub struct UserGateway{
    pub db: Box<DbConn>,
}

impl UserGateway {
    pub fn new(db: Box<DbConn>) -> Self {
        UserGateway {
            db,
        }
    }
}

#[async_trait]
impl UserReader for UserGateway {
    async fn get_user_by_id(&self, user_id: &UserId) -> Option<UserDomain> {
        users::Entity::find_by_id(user_id.clone()).one(&*self.db).await.map(
            |user| Some(map_user_model_to_domain(user.unwrap()))
        ).unwrap()
    }
    
    async fn get_user_by_username_not_sensitive(&self, username: &String) -> Option<UserDomain> {
        let user: Option<users::Model> = users::Entity::find().filter(
                Expr::col(users::Column::Username).ilike(username)
            )
            .one(&*self.db)
            .await
            .unwrap();

        match user {
            Some(user) => Some(map_user_model_to_domain(user)),
            None => None
        }
    }
}

#[async_trait]
impl UserWriter for UserGateway {
    async fn save_user(&self, data: &UserDomain) {
        let user_model = users::ActiveModel {
            id: Set(data.id),
            username: Set(data.username.clone()),
            hashed_password: Set(data.hashed_password.clone()),
            balance: Set(data.balance.clone())
        };

        match users::Entity::find_by_id(data.id).one(&*self.db).await.unwrap() {
            Some(_) => {
                users::Entity::update(user_model).exec(&*self.db).await.unwrap();
            }
            None => {
                users::Entity::insert(user_model).exec(&*self.db).await.unwrap();
            }
        }
    }
}



fn map_user_model_to_domain(user: users::Model) -> UserDomain {
    UserDomain {
        id: user.id,
        username: user.username,
        hashed_password: user.hashed_password,
        balance: user.balance
    }
}

impl UserGatewayTrait for UserGateway {}