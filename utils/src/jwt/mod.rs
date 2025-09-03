use jsonwebtoken::{encode, Header, EncodingKey, DecodingKey, decode, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use std::env;


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,        // subject, 一般为用户 id
    pub exp: usize,         // 过期时间, 单位为秒
}

pub fn generate_token(payload: Claims) -> Result<String, jsonwebtoken::errors::Error> {
    let default_key = "qSe4uXrshGn69EopNqEXnQfNnfrHoZjGVqqMMQ0Ht98=";
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| default_key.to_string());
    let mut header = Header::new(Algorithm::HS256);
    let token = encode(&mut header, &payload, &EncodingKey::from_secret(secret.as_bytes()))?;
    Ok(token)
}

pub fn verify_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let default_key = "qSe4uXrshGn69EopNqEXnQfNnfrHoZjGVqqMMQ0Ht98=";
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| default_key.to_string());
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;
    let token_data = decode::<Claims>(token, &DecodingKey::from_secret(secret.as_bytes()), &validation)?;
    Ok(token_data.claims)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_generate_token() {
        let expire_time = (Utc::now() + chrono::Duration::hours(8)).timestamp() as usize + 3600;
        let payload = Claims {
            sub: "1234567890".to_string(),
            exp: expire_time,
        };

        let token = generate_token(payload).unwrap();
        println!("token: {}", token);
        assert!(!token.is_empty(), "token should not be empty");
        let claims = verify_token(&token).unwrap();
        println!("claims: {:?}", claims);
        assert_eq!(claims.sub, "1234567890");
        assert_eq!(claims.exp, expire_time);
    }
}
