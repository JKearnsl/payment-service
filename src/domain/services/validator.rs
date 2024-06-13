use rust_decimal::prelude::Zero;
use crate::domain::models::money::Money;
use crate::domain::models::session_token::SessionToken;

pub struct ValidatorService {
    username_max_length: usize,
    username_min_length: usize,
    username_regex: regex::Regex,
    password_max_length: usize,
    password_min_length: usize,
    session_token_length: usize,
}

impl ValidatorService {

    pub fn new() -> Self {
        
        // User - - - - - - - - - - - - - - - - - - - - - - - - - - -
        
        // Username
        let username_max_length = 32;
        let username_min_length = 4;
        let username_regex = regex::Regex::new(r"^[a-zA-Z0-9._]*$").unwrap();

        // Password
        let password_max_length = 32;
        let password_min_length = 8;
        
        // Session 
        
        // let session_token_length = 64;
        let session_token_length = 128;
        
        ValidatorService {
            username_max_length,
            username_min_length,
            username_regex,
            password_max_length,
            password_min_length,
            session_token_length,
        }
    }


    pub fn validate_username(&self, username: &str) -> Result<(), String> {

        if username.len() < self.username_min_length || username.len() > self.username_max_length {
            return Err(
                format!(
                    "Имя пользователя должно содержать от {} до {} символов", 
                    self.username_min_length, 
                    self.username_max_length
                )
            );
        }

        if !self.username_regex.is_match(username) {
            return Err(
                "Имя пользователя может содержать только буквы, \
                цифры, точки и символы подчеркивания".to_string()
            );
        }
        Ok(())
    }

    pub fn validate_password(&self, password: &str) -> Result<(), String> {

        if password.len() < self.password_min_length || password.len() > self.password_max_length {
            return Err(
                format!(
                    "Пароль должен содержать от {} до {} символов", 
                    self.password_min_length, 
                    self.password_max_length
                )
            );
        }

        if !password.chars().any(char::is_numeric) {
            return Err("Пароль должен содержать хотя бы одну цифру".to_string());
        }

        if !password.chars().any(char::is_alphabetic) {
            return Err("Пароль должен содержать хотя бы одну букву".to_string());
        }

        if password.chars().any(char::is_whitespace) {
            return Err("Пароль не должен содержать пробелов".to_string());
        }

        Ok(())
    }
    
    pub fn validate_session_token(&self, session_token: &SessionToken) -> Result<(), String> {
        if session_token.len() != self.session_token_length {
            return Err("Неверный формат токена сессии".to_string());
        }
        Ok(())
    }

    pub fn validate_payment_amount(&self, value: &Money) -> Result<(), String> {
        if value.is_zero(){
            return Err("Сумма платежа должна быть больше нуля".to_string());
        }
        
        if value.scale() > 2 {
            return Err("Сумма платежа должна содержать не более двух знаков после запятой".to_string());
        }
        
        if value.is_sign_negative() {
            return Err("Сумма платежа не может быть отрицательной".to_string());
        }
        
        if value > &Money::new(1_000_000_000, 0) {
            return Err("Сумма платежа не может превышать 1 000 000 000".to_string());
        }
        
        Ok(())
    }
    
}
