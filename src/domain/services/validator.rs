use crate::domain::models::money::Money;

pub struct ValidatorService {
    username_max_length: usize,
    username_min_length: usize,
    username_regex: regex::Regex,
    password_max_length: usize,
    password_min_length: usize,
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
        
        
        ValidatorService {
            username_max_length,
            username_min_length,
            username_regex,
            password_max_length,
            password_min_length,
        }
    }


    pub fn validate_username(&self, username: &str) -> Result<(), String> {

        if username.len() < self.username_min_length || username.len() > self.username_max_length {
            return Err(
                format!(
                    "The username must contain between {} and {} characters", 
                    self.username_min_length, 
                    self.username_max_length
                )
            );
        }

        if !self.username_regex.is_match(username) {
            return Err(
                "The username can only contain letters, \
                 numbers, periods and underscores".to_string()
            );
        }
        Ok(())
    }

    pub fn validate_password(&self, password: &str) -> Result<(), String> {

        if password.len() < self.password_min_length || password.len() > self.password_max_length {
            return Err(
                format!(
                    "The password must contain from {} to {} characters", 
                    self.password_min_length, 
                    self.password_max_length
                )
            );
        }

        if !password.chars().any(char::is_numeric) {
            return Err("The password must contain at least one number".to_string());
        }

        if !password.chars().any(char::is_alphabetic) {
            return Err("The password must contain at least one letter".to_string());
        }

        if password.chars().any(char::is_whitespace) {
            return Err("The password must not contain spaces".to_string());
        }

        Ok(())
    }
    
    pub fn validate_payment_amount(&self, value: &Money) -> Result<(), String> {
        if value.is_zero(){
            return Err("The payment amount must be greater than zero".to_string());
        }
        
        if value.scale() > 2 {
            return Err("The payment amount must contain no more than two decimal places".to_string());
        }
        
        if value.is_sign_negative() {
            return Err("The payment amount cannot be negative".to_string());
        }
        
        if value > &Money::new(1_000_000_000, 0) {
            return Err("The payment amount cannot exceed 1,000,000,000".to_string());
        }
        
        Ok(())
    }
    
}
