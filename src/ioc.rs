use sea_orm::DbConn;

use crate::adapters::argon2_password_hasher::Argon2PasswordHasher;
use crate::adapters::argon2_session_hasher::Argon2SessionHasher;
use crate::adapters::database::payment_db::PaymentGateway;
use crate::adapters::database::token_db::TokenGateway;
use crate::adapters::database::user_db::UserGateway;
use crate::adapters::sha256_token_hasher::Sha256TokenHasher;
use crate::application::common::id_provider::IdProvider;
use crate::application::payment::create::CreatePayment;
use crate::application::payment::get_by_id::GetPayment;
use crate::application::payment::get_list::GetPaymentList;
use crate::application::session::create::CreateSession;
use crate::application::session::delete_self::DeleteSessionSelf;
use crate::application::token::create::CreateToken;
use crate::application::token::get_list::GetTokenList;
use crate::application::user::create::CreateUser;
use crate::application::user::get_by_id::GetUserById;
use crate::application::user::get_self::GetUserSelf;
use crate::domain::services::validator::ValidatorService;
use crate::presentation::interactor_factory::InteractorFactory;

pub struct IoC {
    user_gateway: UserGateway,
    token_gateway: TokenGateway,
    payment_gateway: PaymentGateway,
    password_hasher: Argon2PasswordHasher,
    session_hasher: Argon2SessionHasher,
    validator: ValidatorService,
    token_hasher: Sha256TokenHasher,
}

impl IoC {
    pub fn new(
        db_pool: Box<DbConn>,
    ) -> IoC {
        IoC {
            user_gateway: UserGateway::new(db_pool.clone()),
            token_gateway: TokenGateway::new(db_pool.clone()),
            payment_gateway: PaymentGateway::new(db_pool.clone()),
            password_hasher: Argon2PasswordHasher::new(),
            session_hasher: Argon2SessionHasher::new(),
            validator: ValidatorService::new(),
            token_hasher: Sha256TokenHasher {},
        }
    }
}

impl InteractorFactory for IoC {
    
    fn get_user_self(&self, id_provider: Box<dyn IdProvider>) -> GetUserSelf {
        GetUserSelf {
            user_reader: &self.user_gateway,
            id_provider,
        }
    }

    fn get_user_by_id(&self) -> GetUserById {
        GetUserById {
            user_reader: &self.user_gateway,
        }
    }

    fn create_user(&self, id_provider: Box<dyn IdProvider>) -> CreateUser {
        CreateUser {
            user_gateway: &self.user_gateway,
            password_hasher: &self.password_hasher,
            validator: &self.validator,
            id_provider,
        }
    }

    fn create_session(&self, id_provider: Box<dyn IdProvider>) -> CreateSession {
        CreateSession {
            id_provider,
            user_gateway: &self.user_gateway,
            password_hasher: &self.password_hasher,
            session_hasher: &self.session_hasher,
            validator: &self.validator,
        }
    }

    fn delete_self_session(&self, id_provider: Box<dyn IdProvider>) -> DeleteSessionSelf {
        DeleteSessionSelf {
            id_provider,
        }
    }

    fn create_payment(&self) -> CreatePayment {
        CreatePayment {
            payment_gateway: &self.payment_gateway,
            token_reader: &self.token_gateway,
            validator: &self.validator,
            password_hasher: &self.password_hasher,
            token_hasher: &self.session_hasher,
        }
    }

    fn get_payment(&self) -> GetPayment {
        GetPayment {
            payment_reader: &self.payment_gateway,
        }
    }

    fn get_payments(&self, id_provider: Box<dyn IdProvider>) -> GetPaymentList {
        GetPaymentList {
            payment_reader: &self.payment_gateway,
            id_provider,
        }
    }

    fn create_token(&self, id_provider: Box<dyn IdProvider>) -> CreateToken {
        CreateToken {
            token_gateway: &self.token_gateway,
            token_hasher: &self.token_hasher,
            validator: &self.validator,
            id_provider,
        }
    }

    fn get_tokens(&self, id_provider: Box<dyn IdProvider>) -> GetTokenList {
        GetTokenList {
            token_reader: &self.token_gateway,
            id_provider,
        }
    }
}