use std::str::FromStr;
use actix_web::{HttpRequest, web};
use crate::adapters::auth::token::IdTokenProvider;
use crate::application::common::id_provider::IdProvider;
use crate::application::common::token_processor::TokenProcessor;
use crate::domain::models::session_token::SessionToken;

pub fn get_id_provider(
    req: &HttpRequest,
    token_processor: &web::Data<dyn TokenProcessor>
) -> Box<dyn IdProvider> {
    let session_token = req.cookie("session_token")
        .map(|cookie| SessionToken::from_str(cookie.value()));
    
    IdTokenProvider::new(
        session_token, 
        token_processor
    )
}
