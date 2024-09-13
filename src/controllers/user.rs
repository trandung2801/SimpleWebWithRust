use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Result;
use argon2::Config;
use rand::random;
use tracing::instrument;
use warp::http::StatusCode;

use crate::errors::Error;
use crate::models::pagination::Pagination;
use crate::models::role::{RoleId, ADMIN_ROLE_ID, HR_ROLE_ID};
use crate::models::store_trait::StoreMethods;
use crate::models::user::{AuthInfo, User, UserId, UserInfo};
use crate::services::jwt::{Claims, Jwt, JwtActions};
use crate::utils::convert_to_json::{Data, PayloadForLogin, PayloadNoData, PayloadWithData};

pub fn hash_password(password: &[u8]) -> String {
    let salt = random::<[u8; 32]>();
    let config = Config::default();
    argon2::hash_encoded(password, &salt, &config).unwrap()
}
pub fn verify_password(hash: &str, password: &[u8]) -> Result<bool, argon2::Error> {
    argon2::verify_encoded(hash, password)
}
// Convert User Object to UserInfo Object for hide password when returning response
fn convert_user_to_user_info(user: User) -> UserInfo {
    UserInfo {
        id: user.id.unwrap(),
        email: user.email,
        company_id: user.company_id,
        role_id: user.role_id,
        is_delete: user.is_delete,
    }
}
// Handle for register user by email and password
#[instrument(level = "info", skip(store))]
pub async fn register(
    store: Arc<dyn StoreMethods + Send + Sync>,
    new_user: AuthInfo,
) -> Result<impl warp::Reply, warp::Rejection> {
    // Check valid user
    let new_email = new_user.email;
    if let Ok(_res) = store.get_user_by_email(new_email.clone()).await {
        let payload = PayloadNoData {
            message: "Email already exists".to_string(),
        };
        return Ok(warp::reply::with_status(
            warp::reply::json(&payload),
            StatusCode::BAD_REQUEST,
        ));
    }
    let hash_password = hash_password(new_user.hash_password.as_bytes());
    let user = AuthInfo {
        email: new_email,
        hash_password,
    };
    let res = store.create_user(user).await.map_err(Error::from)?;
    let user_info = convert_user_to_user_info(res);
    let payload = PayloadWithData {
        message: "Success".to_string(),
        data: Data::UserInfo(user_info),
    };
    Ok(warp::reply::with_status(
        warp::reply::json(&payload),
        StatusCode::CREATED,
    ))
}

// Handle for login system by email and password
#[instrument(level = "info", skip(store))]
pub async fn login(
    store: Arc<dyn StoreMethods + Send + Sync>,
    login_info: AuthInfo,
) -> Result<impl warp::Reply, warp::Rejection> {
    // Login
    let user = store.get_user_by_email(login_info.email.clone()).await?;
    let verified = verify_password(&user.hash_password, login_info.hash_password.as_bytes())
        .map_err(Error::from)?;
    if verified {
        let token = Jwt::issue_access_token(user.clone()).unwrap();
        let user_info = convert_user_to_user_info(user);
        let payload = PayloadForLogin {
            access_token: token,
            message: "Login success".to_string(),
            data: Data::UserInfo(user_info),
        };
        Ok(warp::reply::with_status(
            warp::reply::json(&payload),
            StatusCode::OK,
        ))
    } else {
        Err(warp::reject::custom(Error::WrongPassword))
    }
}

// Handle for retrieving user by ID
#[instrument(level = "info", skip(store))]
pub async fn get_user_by_id(
    store: Arc<dyn StoreMethods + Send + Sync>,
    user_id: i32,
) -> Result<impl warp::Reply, warp::Rejection> {
    let res = store
        .get_user_by_id(UserId(user_id))
        .await
        .map_err(Error::from)?;
    let user_info = convert_user_to_user_info(res);
    let payload = PayloadWithData {
        message: "Success".to_string(),
        data: Data::UserInfo(user_info),
    };
    Ok(warp::reply::with_status(
        warp::reply::json(&payload),
        StatusCode::OK,
    ))
}

// Handle for retrieving list users based on query parameters
#[instrument(level = "info", skip(store))]
pub async fn get_list_users(
    store: Arc<dyn StoreMethods + Send + Sync>,
    params: HashMap<String, String>,
) -> Result<impl warp::Reply, warp::Rejection> {
    // Get pagination from query parameters
    let mut pagination = Pagination::default();

    if !params.is_empty() {
        pagination = Pagination::extract_pagination(params)?;
    }
    // Get list users with pagination filters
    let res = store
        .get_list_user(pagination.limit, pagination.offset)
        .await
        .map_err(Error::from)?;
    let mut list_user_info = Vec::new();
    for element in res {
        let user_info = convert_user_to_user_info(element);
        list_user_info.push(user_info);
    }
    let payload = PayloadWithData {
        message: "Success".to_string(),
        data: Data::ListUserInfo(list_user_info),
    };
    Ok(warp::reply::with_status(
        warp::reply::json(&payload),
        StatusCode::OK,
    ))
}

// Handler for updating user.
#[instrument(level = "info", skip(store))]
pub async fn update_user(
    store: Arc<dyn StoreMethods + Send + Sync>,
    claims: Claims,
    user_update: UserInfo,
) -> Result<impl warp::Reply, warp::Rejection> {
    // Check id of user sending request and id of UserInfo sent from user
    if claims.id != user_update.id {
        return Err(warp::reject::custom(Error::Unauthenticated));
    };
    let res = store.update_user(user_update).await.map_err(Error::from)?;
    let user_info = convert_user_to_user_info(res);
    let payload = PayloadWithData {
        message: "Success".to_string(),
        data: Data::UserInfo(user_info),
    };
    Ok(warp::reply::with_status(
        warp::reply::json(&payload),
        StatusCode::OK,
    ))
}

// Handler for updating password of user.
#[instrument(level = "info", skip(store))]
pub async fn update_password(
    store: Arc<dyn StoreMethods + Send + Sync>,
    claims: Claims,
    user_update: AuthInfo,
) -> Result<impl warp::Reply, warp::Rejection> {
    // Check email of user sending request and email of AuthInfo sent from user
    if claims.email != user_update.email {
        return Err(warp::reject::custom(Error::Unauthenticated));
    };
    let hash_password = hash_password(user_update.hash_password.as_bytes());
    let user = AuthInfo {
        email: user_update.email,
        hash_password,
    };
    let res = store.update_password(user).await.map_err(Error::from)?;
    let user_info = convert_user_to_user_info(res);
    let payload = PayloadWithData {
        message: "Success".to_string(),
        data: Data::UserInfo(user_info),
    };
    Ok(warp::reply::with_status(
        warp::reply::json(&payload),
        StatusCode::OK,
    ))
}

// 2 function handler for updating role of user.
#[instrument(level = "info", skip(store))]
pub async fn set_admin_role(
    store: Arc<dyn StoreMethods + Send + Sync>,
    claims: Claims,
    user: UserInfo,
) -> Result<impl warp::Reply, warp::Rejection> {
    let res = store
        .set_role(user, RoleId(ADMIN_ROLE_ID))
        .await
        .map_err(Error::from)?;
    let user_info = convert_user_to_user_info(res);
    let payload = PayloadWithData {
        message: "Success".to_string(),
        data: Data::UserInfo(user_info),
    };
    Ok(warp::reply::with_status(
        warp::reply::json(&payload),
        StatusCode::OK,
    ))
}

#[instrument(level = "info", skip(store))]
pub async fn set_hr_role(
    store: Arc<dyn StoreMethods + Send + Sync>,
    claims: Claims,
    user: UserInfo,
) -> Result<impl warp::Reply, warp::Rejection> {
    let res = store
        .set_role(user, RoleId(HR_ROLE_ID))
        .await
        .map_err(Error::from)?;
    let user_info = convert_user_to_user_info(res);
    let payload = PayloadWithData {
        message: "Success".to_string(),
        data: Data::UserInfo(user_info),
    };
    Ok(warp::reply::json(&payload))
}

// Handler for deleting user by ID.
#[instrument(level = "info", skip(store))]
pub async fn delete(
    store: Arc<dyn StoreMethods + Send + Sync>,
    claims: Claims,
    user_delete: UserInfo,
) -> Result<impl warp::Reply, warp::Rejection> {
    // Check id of user sending request and id of UserInfo sent from user
    if claims.id != user_delete.id {
        return Err(warp::reject::custom(Error::Unauthenticated));
    };
    let _ = store
        .delete_user_by_id(user_delete.id)
        .await
        .map_err(Error::from)?;
    Ok(warp::reply::with_status(
        "Success".to_string(),
        StatusCode::OK,
    ))
}
