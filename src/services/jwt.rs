use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use tracing::{event, Level};

use crate::errors::Error;
use crate::models::role::RoleId;
use crate::models::user::{User, UserId};

const JWT_ACCESS_TOKEN_SECRET: &[u8] = b"access secret";
const JWT_ACCESS_TOKEN_IN: i64 = 1;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Claims {
    pub id: UserId,
    pub email: String,
    pub role_id: RoleId,
    pub is_delete: bool,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Jwt;

pub trait JwtActions {
    fn issue_access_token(user: User) -> Result<String, Error>;
    fn verify_access_token(token: &str) -> Result<Claims, Error>;
}

impl JwtActions for Jwt {
    fn issue_access_token(user: User) -> Result<String, Error> {
        // Create claim for the token
        let current_date_time = Utc::now();
        let claim_iat = current_date_time.timestamp() as usize;
        let claim_exp =
            (current_date_time + chrono::Duration::hours(JWT_ACCESS_TOKEN_IN)).timestamp() as usize;
        // let exp = (current_date_time + chrono::Duration::minutes(JWT_ACCESS_TOKEN_IN)).timestamp() as usize;

        let claim = Claims {
            id: user.id.unwrap(),
            email: user.email,
            role_id: user.role_id,
            is_delete: user.is_delete,
            iat: claim_iat,
            exp: claim_exp,
        };
        // Set algorithm hash for jwt token
        let header = Header::new(Algorithm::HS512);
        match encode(
            &header,
            &claim,
            &EncodingKey::from_secret(JWT_ACCESS_TOKEN_SECRET),
        ) {
            Ok(token) => Ok(token),
            Err(e) => {
                event!(Level::ERROR, "Encode from claim has error: {:?}", e);
                Err(Error::CannotEncryptToken)
            }
        }
    }

    fn verify_access_token(token: &str) -> Result<Claims, Error> {
        match decode::<Claims>(
            token,
            &DecodingKey::from_secret(JWT_ACCESS_TOKEN_SECRET),
            &Validation::new(Algorithm::HS512),
        ) {
            Ok(token_data) => Ok(token_data.claims),
            Err(e) => {
                event!(
                    Level::ERROR,
                    "Decode access token from token has error: {:?}",
                    e
                );
                Err(Error::CannotDecryptToken)
            }
        }
    }
}
