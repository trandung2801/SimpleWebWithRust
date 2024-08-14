use std::collections::HashMap;
use std::sync::Arc;
use argon2::Config;
use rand::Rng;
use serde_json::json;
use tracing::{event, instrument, Level};
use handle_errors::Error;
use warp::http::StatusCode;
use warp::Filter;
use crate::middleware::convert_to_json::{PayloadNoData, PayloadWithData, Data, PayloadForLogin};
use crate::middleware::jwt::{Jwt, Claims, JwtActions};
use crate::models::pagination::{Pagination, PaginationMethods};
use crate::models::role::{ADMIN_ROLE_ID, HR_ROLE_ID, RoleId};
use crate::models::user::{UserInfo, AuthInfo, UserId, User};
use crate::models::store_db::{Store};
use crate::models::store_trait::StoreMethods;

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

fn convert_user_to_user_info(user: User) -> UserInfo {
    UserInfo{
        id: user.id.unwrap(),
        email: user.email,
        company_id: user.company_id,
        role_id: user.role_id,
        is_delete: user.is_delete
    }
}
#[instrument(level = "info")]
pub async fn register(store: Arc<dyn StoreMethods + Send + Sync>, new_user: AuthInfo)
    -> Result<impl warp::Reply, warp::Rejection>
{
    let new_email = new_user.email;
    match store.get_user_by_email(&new_email).await {
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
    match store.create_user(user).await {
        Ok(res) =>
            {
                let user_info = convert_user_to_user_info(res);
                let payload = PayloadWithData {
                    message: "Register success".to_string(),
                    data: Data::UserInfo(user_info),
                };
                Ok(warp::reply::with_status(warp::reply::json(&payload), StatusCode::CREATED))
            }
        Err(e) => Err(warp::reject::custom(e)),
    }
}

#[instrument(level = "info")]
pub async fn login(store: Arc<dyn StoreMethods + Send + Sync>, login_info: AuthInfo)
    -> Result<impl warp::Reply, warp::Rejection>
{
    match store.get_user_by_email(&login_info.email).await {
        Ok(user) => match verify_password(
            &user.password,
            login_info.password.as_bytes(),
        ) {
            Ok(verified) => {
                if verified {
                    match Jwt::issue_access_token(user.clone()) {
                        Ok(access_token) => {
                            let _user = store.get_user_by_id(user.id.unwrap()).await?;
                            let user_info = convert_user_to_user_info(_user);
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

#[instrument(level = "info")]
pub async fn get_user_by_id(store: Arc<dyn StoreMethods + Send + Sync>, user_id: i32)
                            -> Result<impl warp::Reply, warp::Rejection>
{
    println!("user_id: {}", user_id);
    event!(target: "backend", Level::INFO, "querying user");
    match store.get_user_by_id(UserId(user_id)).await {
        Ok(res) =>
            {
                let user_info = convert_user_to_user_info(res);
                let payload = PayloadWithData {
                    message: "Get user success".to_string(),
                    data: Data::UserInfo(user_info)
                };
                Ok(warp::reply::with_status(warp::reply::json(&payload), StatusCode::OK))
            }
        Err(e) => Err(warp::reject()),
    }
}

#[instrument(level = "info")]
pub async fn get_list_users(store: Arc<dyn StoreMethods + Send + Sync>, params: HashMap<String, String>)
    -> Result<impl warp::Reply, warp::Rejection>
{
    let mut pagination = Pagination::default();

    if !params.is_empty() {
        event!(Level::INFO, pagination = true);
        pagination = <Pagination as PaginationMethods>::extract_pagination(params)?;
    }
    match store.get_list_user(pagination.limit, pagination.offset).await {
        Ok(res) =>
            {
                let mut list_user_info = Vec::new();
                for e in res {
                    let user_info = convert_user_to_user_info(e);
                    list_user_info.push(user_info);
                    }
                let payload = PayloadWithData {
                    message: "Get list user success".to_string(),
                    data: Data::ListUserInfo(list_user_info)
                };
                Ok(warp::reply::with_status(warp::reply::json(&payload), StatusCode::OK))
            }
        Err(e) => Err(warp::reject()),
    }
}

#[instrument(level = "info")]
pub async fn update_user(store: Arc<dyn StoreMethods + Send + Sync>, claims: Claims, user_update: UserInfo)
    -> Result<impl warp::Reply, warp::Rejection>
{
    // valid user
    if claims.id != user_update.id {
        return Err(warp::reject())
    };
    match store.update_user(user_update).await {
        Ok(res) =>
            {
                let user_info = convert_user_to_user_info(res);
                let payload = PayloadWithData {
                    message: "Update user success".to_string(),
                    data: Data::UserInfo(user_info)
                };
                Ok(warp::reply::with_status(warp::reply::json(&payload), StatusCode::OK))
            }
        Err(e) => Err(warp::reject::custom(e)),
    }
}

#[instrument(level = "info")]
pub async fn update_password(store: Arc<dyn StoreMethods + Send + Sync>, claims: Claims, user_update: AuthInfo)
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
    match store.update_password(user).await {
        Ok(res) =>
            {
                let user_info = convert_user_to_user_info(res);
                let payload = PayloadWithData {
                    message: "Update password success".to_string(),
                    data: Data::UserInfo(user_info)
                };
                Ok(warp::reply::with_status(warp::reply::json(&payload), StatusCode::OK))
            }
        Err(e) => Err(warp::reject::custom(e)),
    }
}

#[instrument(level = "info")]
pub async fn set_admin_role(store: Arc<dyn StoreMethods + Send + Sync>, claims: Claims, user: UserInfo)
                                        -> Result<impl warp::Reply, warp::Rejection>
{
    match store.set_role(user, RoleId(ADMIN_ROLE_ID)).await {
        Ok(res) =>
            {
                let user_info = convert_user_to_user_info(res);
                let payload = PayloadWithData {
                    message: "Update user success".to_string(),
                    data: Data::UserInfo(user_info)
                };
                Ok(warp::reply::with_status(warp::reply::json(&payload), StatusCode::OK))
            }
        Err(e) => Err(warp::reject::custom(e)),
    }
}

#[instrument(level = "info")]
pub async fn set_hr_role(store: Arc<dyn StoreMethods + Send + Sync>, claims: Claims, user: UserInfo)
                            -> Result<impl warp::Reply, warp::Rejection>
{
    match store.set_role(user, RoleId(HR_ROLE_ID)).await {
        Ok(res) =>
            {
                let user_info = convert_user_to_user_info(res);
                let payload = PayloadWithData {
                    message: "Update user success".to_string(),
                    data: Data::UserInfo(user_info)
                };
                Ok(warp::reply::json(&payload))
            }
        Err(e) => Err(warp::reject::custom(e)),
    }
}

#[instrument(level = "info")]
pub async fn delete(store: Arc<dyn StoreMethods + Send + Sync>, claims: Claims, user_delete: UserInfo)
    -> Result<impl warp::Reply, warp::Rejection>
{
    if claims.id != user_delete.id {
        return Err(warp::reject())
    };
    match store.delete_user_by_id(user_delete.id).await {
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



