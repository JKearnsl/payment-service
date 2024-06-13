use crate::application::common::id_provider::IdProvider;
use crate::application::payment::create::CreatePayment;
use crate::application::payment::get_by_id::GetPayment;
use crate::application::payment::get_list::GetPaymentList;
use crate::application::session::create::CreateSession;
use crate::application::session::delete_self::DeleteSessionSelf;
use crate::application::token::create::CreateToken;
use crate::application::token::get_list::GetTokenList;
use crate::application::user::get_by_id::GetUserById;
use crate::application::user::create::CreateUser;
use crate::application::user::get_self::GetUserSelf;

pub trait InteractorFactory {
    fn get_user_self(&self, id_provider: Box<dyn IdProvider>) -> GetUserSelf;
    fn get_user_by_id(&self) -> GetUserById;
    fn create_user(&self, id_provider: Box<dyn IdProvider>) -> CreateUser;
    fn create_session(&self, id_provider: Box<dyn IdProvider>) -> CreateSession;
    fn delete_self_session(&self, id_provider: Box<dyn IdProvider>) -> DeleteSessionSelf;
    fn create_payment(&self) -> CreatePayment;
    fn get_payment(&self) -> GetPayment;
    fn get_payments(&self, id_provider: Box<dyn IdProvider>) -> GetPaymentList;
    fn create_token(&self, id_provider: Box<dyn IdProvider>) -> CreateToken;
    fn get_tokens(&self, id_provider: Box<dyn IdProvider>) -> GetTokenList;
}
