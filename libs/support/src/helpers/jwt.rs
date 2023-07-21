use std::vec;
use chrono::Utc;
use error::Error;
use serde::{Deserialize, Serialize};
use crate::store::models::user::AuthenticatedUser;
use crate::store::models::user;

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub exp: i64,
    pub iat: i64,
}

/// Function that generates JWT token
pub fn generate(user: &AuthenticatedUser) -> Result<String, Error> {
    let mut jwt_params = 
        config::get_multiple_default(
            vec![("JWT_SECRET", "not_so_strong_secret"), ("JWT_LIFETIME_IN_SECONDS", "600")]
        );

    let jwt_lifetime: i64 = jwt_params.pop().unwrap().parse().unwrap();
    let jwt_secret = jwt_params.pop().unwrap();
    let exp = Utc::now() + chrono::Duration::seconds(jwt_lifetime);

    let claims = Claims {
        sub: String::from(&user.id),
        email: String::from(&user.email),
        exp: exp.timestamp(),
        iat: Utc::now().timestamp(),
    };

    jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
    .map_err(|_e| Error::Jwt)
}

/// Verify given token and return user if its okay
pub fn verify(token: String, secret: &str) -> Result<Claims, Error> {
    let token_data = jsonwebtoken::decode::<Claims>(
        &token,
        &jsonwebtoken::DecodingKey::from_secret(secret.as_bytes()),
        &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256),
    )
    .map_err(|_e| Error::Jwt)?;
    
    Ok(token_data.claims)
}

/// Create testable jwt token
pub fn testable() -> String {
    let user = user::testable("test@test.com", None, None, None, None);
    let authenticated_user = AuthenticatedUser::from(user);
    
    generate(&authenticated_user).unwrap()
}