use std::collections::HashMap;
use std::sync::Arc;
use argon2::Config;
use rand::Rng;
use serde_json::json;
use tokio::join;
use tracing::{event, instrument, Level};
use handle_errors::Error;
use warp::http::StatusCode;
use crate::middleware::jwt::{Jwt, Claims};
use crate::models::user::{UserInfo, User, UserMac, AuthInfo};
use crate::models::store::Store;

pub fn hash_password(password: &[u8])
                     -> String
{
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();
    argon2::hash_encoded(password, &salt, &config).unwrap()
}
pub fn verify_password(hash: &str, password: &[u8])
                       -> Result<bool, argon2::Error>
{
    argon2::verify_encoded(hash, password)
}

pub async fn register(store: Store, new_user: AuthInfo)
    -> Result<impl warp::Reply, warp::Rejection>
{
    let new_email = new_user.email;
    match UserMac::get(store.clone(), &new_email).await {
        Ok(res) => {
            let payload = json!({
                "statusCode": 201,
                "message": "Email invalid",
            });
            return Ok(warp::reply::json(&payload))
        }
        _ => ()
    }
    let hash_password = hash_password(new_user.password.as_bytes());
    let user = AuthInfo {
        email: new_email,
        password: hash_password,
    };
    match UserMac::create(store, user).await {
        Ok(res) =>
            {
                let payload = json!({
                    "statusCode": 201,
                    "message": "Register success",
                    "data": res
                });
                Ok(warp::reply::json(&payload))
            }
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn login(store: Store, login_info: AuthInfo)
    -> Result<impl warp::Reply, warp::Rejection>
{
    match UserMac::get(store, &login_info.email).await {
        Ok(user) => match verify_password(
            &user.password,
            login_info.password.as_bytes(),
        ) {
            Ok(verified) => {
                if verified {
                    match Jwt::issue_access_token(user.clone()) {
                        Ok(access_token) => {
                            let payload = json!({
                                "statusCode": 201,
                                "message": "login success",
                                "accessToken" : access_token,
                                "data": {
                                    "id": user.id,
                                    "email": user.email,
                                    "company": user.company,
                                    "is_admin": user.is_admin
                                }
                            });
                            Ok(warp::reply::json(&payload))
                        }
                        Err(e) => Err(warp::reject::custom(e)),
                    }
                } else {
                    Err(warp::reject::custom(Error::WrongPassword))
                }
            }
            Err(e) => Err(warp::reject::custom(Error::ArgonLibraryError(e))),
        }
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn get_user(store: Store, user_email: String)
    -> Result<impl warp::Reply, warp::Rejection>
{
    event!(target: "backend", Level::INFO, "querying user");
    match UserMac::get(store, &user_email).await {
        Ok(user) =>
            {
                let payload = json!({
                    "statusCode": 201,
                    "message": "get user success",
                     "data": {
                        "id": user.id,
                        "email": user.email,
                        "company": user.company,
                        "is_admin": user.is_admin
                    }
                });
                Ok(warp::reply::json(&payload))
            }
        Err(e) => Err(warp::reject()),
    }
}

pub async fn get_list_users(store: Store)
    -> Result<impl warp::Reply, warp::Rejection>
{
    match UserMac::list(store).await {
        Ok(res) =>
            {
                let payload = json!({
                    "statusCode": 201,
                    "message": "get list users success",
                    "data": res
                });
                Ok(warp::reply::json(&payload))
            }
        Err(e) => Err(warp::reject()),
    }
}


pub async fn update_user(store: Store, claims: Claims, user_update: UserInfo)
    -> Result<impl warp::Reply, warp::Rejection>
{
    if claims.email != user_update.email {
        return Err(warp::reject())
    };
    match UserMac::update_user(store, user_update).await {
        Ok(res) =>
            {
                let payload = json!({
                    "statusCode": 201,
                    "message": "update user success",
                    "data": res
                });
                Ok(warp::reply::json(&payload))
            }
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn update_password(store: Store, claims: Claims, user_update: AuthInfo)
                               -> Result<impl warp::Reply, warp::Rejection>
{
    if claims.email != user_update.email {
        return Err(warp::reject())
    };
    let hash_password = hash_password(user_update.password.as_bytes());
    let user = AuthInfo {
        email: user_update.email,
        password: hash_password
    };
    match UserMac::update_password(store, user).await {
        Ok(res) =>
            {
                let payload = json!({
                    "statusCode": 201,
                    "message": "update password success",
                    "data": res
                });
                Ok(warp::reply::json(&payload))
            }
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn set_admin_role(store: Store, claims: Claims, user: UserInfo)
                                        -> Result<impl warp::Reply, warp::Rejection>
{
    if claims.email != user.email {
        return Err(warp::reject())
    };
    match UserMac::set_admin(store, user).await {
        Ok(res) =>
            {
                let payload = json!({
                    "statusCode": 201,
                    "message": "update admin role success",
                    "data": res
                });
                Ok(warp::reply::json(&payload))
            }
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn delete(store: Store, claims: Claims, user_delete: UserInfo)
    -> Result<impl warp::Reply, warp::Rejection>
{
    if claims.email != user_delete.email {
        return Err(warp::reject())
    };
    match UserMac::delete(store, user_delete.email).await {
        Ok(_) =>
            {
                let payload = json!({
                    "statusCode": 201,
                    "message": "delete user success",
                });
                Ok(warp::reply::json(&payload))
            }
        Err(e) => Err(warp::reject::custom(e)),
    }
}



