use std::collections::HashMap;
use argon2::Config;
use rand::Rng;
use serde_json::json;
use tracing::{event, Level};
use handle_errors::Error;
use warp::http::StatusCode;
use warp::Filter;
use crate::middleware::convert_to_json::{PayloadNoData, PayloadWithData, Data, PayloadForLogin};
use crate::middleware::jwt::{Jwt, Claims, JwtActions};
use crate::models::pagination::{Pagination, PaginationMethods};
use crate::models::role::{ADMIN_ROLE_ID, HR_ROLE_ID, RoleId};
use crate::models::user::{UserInfo, UserMac, AuthInfo, UserActions, UserId};
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
    match UserMac::get_by_email(store.clone(), &new_email).await {
        Ok(res) => {
            let payload = PayloadNoData {
                message: "Email invalid".to_string(),
            };
            return Ok(warp::reply::with_status(warp::reply::json(&payload), StatusCode::BAD_REQUEST))
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
                let payload = PayloadWithData {
                    message: "Register success".to_string(),
                    data: Data::UserInfo(res),
                };
                Ok(warp::reply::with_status(warp::reply::json(&payload), StatusCode::CREATED))
            }
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn login(store: Store, login_info: AuthInfo)
    -> Result<impl warp::Reply, warp::Rejection>
{
    match UserMac::get_by_email(store.clone(), &login_info.email).await {
        Ok(user) => match verify_password(
            &user.password,
            login_info.password.as_bytes(),
        ) {
            Ok(verified) => {
                if verified {
                    match Jwt::issue_access_token(user.clone()) {
                        Ok(access_token) => {
                            let user_info = UserMac::get_by_id(store.clone(), user.id.unwrap()).await?;
                            let payload = PayloadForLogin {
                                access_token: access_token,
                                message: "Login success".to_string(),
                                data: Data::UserInfo(user_info)
                            };
                            Ok(warp::reply::with_status(warp::reply::json(&payload), StatusCode::OK))
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

pub async fn get_user_by_id(store: Store, user_id: i32)
    -> Result<impl warp::Reply, warp::Rejection>
{
    println!("user_id: {}", user_id);
    event!(target: "backend", Level::INFO, "querying user");
    match UserMac::get_by_id(store, UserId(user_id)).await {
        Ok(user) =>
            {
                let payload = PayloadWithData {
                    message: "Get user success".to_string(),
                    data: Data::UserInfo(user)
                };
                Ok(warp::reply::with_status(warp::reply::json(&payload), StatusCode::OK))
            }
        Err(e) => Err(warp::reject()),
    }
}

pub async fn get_list_users(store: Store, params: HashMap<String, String>)
    -> Result<impl warp::Reply, warp::Rejection>
{
    let mut pagination = Pagination::default();

    if !params.is_empty() {
        event!(Level::INFO, pagination = true);
        pagination = <Pagination as PaginationMethods>::extract_pagination(params)?;
    }
    match UserMac::list(store, pagination.limit, pagination.offset).await {
        Ok(res) =>
            {
                let payload = PayloadWithData {
                    message: "Get list user success".to_string(),
                    data: Data::ListUserInfo(res)
                };
                Ok(warp::reply::with_status(warp::reply::json(&payload), StatusCode::OK))
            }
        Err(e) => Err(warp::reject()),
    }
}


pub async fn update_user(store: Store, claims: Claims, user_update: UserInfo)
    -> Result<impl warp::Reply, warp::Rejection>
{
    // valid user
    if claims.id != user_update.id {
        return Err(warp::reject())
    };
    match UserMac::update_user(store, user_update).await {
        Ok(res) =>
            {
                let payload = PayloadWithData {
                    message: "Update user success".to_string(),
                    data: Data::UserInfo(res)
                };
                Ok(warp::reply::with_status(warp::reply::json(&payload), StatusCode::OK))
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
                let payload = PayloadWithData {
                    message: "Update password success".to_string(),
                    data: Data::UserInfo(res)
                };
                Ok(warp::reply::with_status(warp::reply::json(&payload), StatusCode::OK))
            }
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn set_admin_role(store: Store, claims: Claims, user: UserInfo)
                                        -> Result<impl warp::Reply, warp::Rejection>
{
    match UserMac::set_role(store, user, RoleId(ADMIN_ROLE_ID)).await {
        Ok(res) =>
            {
                let payload = PayloadWithData {
                    // status_code: StatusCode::OK,
                    message: "Update user success".to_string(),
                    data: Data::UserInfo(res)
                };
                Ok(warp::reply::with_status(warp::reply::json(&payload), StatusCode::OK))
            }
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn set_hr_role(store: Store, claims: Claims, user: UserInfo)
                            -> Result<impl warp::Reply, warp::Rejection>
{
    match UserMac::set_role(store, user, RoleId(HR_ROLE_ID)).await {
        Ok(res) =>
            {
                let payload = PayloadWithData {
                    message: "Update user success".to_string(),
                    data: Data::UserInfo(res)
                };
                Ok(warp::reply::json(&payload))
            }
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn delete(store: Store, claims: Claims, user_delete: UserInfo)
    -> Result<impl warp::Reply, warp::Rejection>
{
    if claims.id != user_delete.id {
        return Err(warp::reject())
    };
    match UserMac::delete(store, user_delete.id).await {
        Ok(_) =>
            {
                let payload = PayloadNoData {
                    message: "Delete User Success".to_string(),
                };
                Ok(warp::reply::with_status(warp::reply::json(&payload), StatusCode::OK))
            }
        Err(e) => Err(warp::reject::custom(e)),
    }
}



