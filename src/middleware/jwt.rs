use chrono::Utc;
use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use handle_errors::{Error};
use crate::models::role::RoleId;
use crate::models::user::{User, UserId};

const JWT_ACCESS_TOKEN_SECRET: &[u8] = b"access secret";
const JWT_ACCESS_TOKEN_IN: i64 = 1;
const JWT_REFRESH_TOKEN_SECRET: &[u8] = b"secret";
const JWT_REFRESH_TOKEN_IN: i64 = 24;


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Claims {
    pub id: UserId,
    pub email: String,
    pub role: RoleId,
    pub is_delete: bool,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Jwt;

pub trait JwtActions {
    fn issue_refresh_token(user: User)
                               -> Result<String, Error>;
    fn verify_refresh_token(token: &str)
                            -> Result<Claims, Error>;
    fn issue_access_token(user: User)
                          -> Result<String, Error>;
    fn verify_access_token(token: &str)
                           -> Result<Claims, Error>;
}

impl JwtActions for Jwt {
    fn issue_refresh_token(user: User)
                               -> Result<String, Error>
    {
        let current_date_time = Utc::now();
        let iat = current_date_time.timestamp() as usize;
        let exp = (current_date_time + chrono::Duration::hours(JWT_REFRESH_TOKEN_IN)).timestamp() as usize;

        let claim = Claims {
            id: user.id.unwrap(),
            email: user.email,
            role: user.role_id,
            is_delete: user.is_delete,
            iat: iat,
            exp: exp
        };

        let header = Header::new(Algorithm::HS512);
        match encode(&header,
                     &claim,
                     &EncodingKey::from_secret(JWT_REFRESH_TOKEN_SECRET))
        {
            Ok(token) => Ok(token),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::CannotEncryptToken)
            }
        }
    }

    fn verify_refresh_token(token: &str)
                                -> Result<Claims, Error>
    {
        match decode::<Claims>(
            &token,
            &DecodingKey::from_secret(JWT_REFRESH_TOKEN_SECRET),
            &Validation::new(Algorithm::HS512),
        ) {
            Ok(token_data) => Ok(token_data.claims),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::CannotDecryptToken)
            }
        }
    }

    fn issue_access_token(user: User)
                              -> Result<String, Error>
    {
        let current_date_time = Utc::now();
        let iat = current_date_time.timestamp() as usize;
        let exp = (current_date_time + chrono::Duration::hours(JWT_ACCESS_TOKEN_IN)).timestamp() as usize;
        // let exp = (current_date_time + chrono::Duration::minutes(JWT_ACCESS_TOKEN_IN)).timestamp() as usize;

        let claim = Claims {
            id: user.id.unwrap(),
            email: user.email,
            role: user.role_id,
            is_delete:user.is_delete,
            iat: iat,
            exp: exp
        };

        let header = Header::new(Algorithm::HS512);
        match encode(&header,
                     &claim,
                     &EncodingKey::from_secret(JWT_ACCESS_TOKEN_SECRET))
        {
            Ok(token) => Ok(token),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::CannotEncryptToken)
            }
        }
    }

    fn verify_access_token(token: &str)
                               -> Result<Claims, Error>
    {
        match decode::<Claims>(
            &token,
            &DecodingKey::from_secret(JWT_ACCESS_TOKEN_SECRET),
            &Validation::new(Algorithm::HS512),
        ) {
            Ok(token_data) => Ok(token_data.claims),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::CannotDecryptToken)
            }
        }
    }
}





