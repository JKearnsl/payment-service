use std::str::FromStr;
use actix_web::{HttpRequest, web};
use crate::adapters::auth::session::IdSessionProvider;
use crate::application::common::id_provider::IdProvider;
use crate::application::common::session_processor::SessionProcessor;
use crate::domain::models::session_token::SessionToken;

pub async fn get_id_session_provider(
    req: &HttpRequest,
    session_processor: &web::Data<dyn SessionProcessor>
) -> Box<dyn IdProvider> {
    let session_token = req.cookie("session_token")
        .map(|cookie| SessionToken::from_str(cookie.value()).unwrap());
    
    let processor = session_processor.get_ref();
    
    Box::new(
        IdSessionProvider::new(
            session_token,
            processor
        ).await
    )
}
