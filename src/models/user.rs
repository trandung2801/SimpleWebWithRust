use serde::{Deserialize, Serialize};
use handle_errors::Error;
use crate::models::company::CompanyId;
use crate::models::role::RoleId;
use crate::models::store_impl_user::UserStoreMethods;
use super::store::Store;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    pub id: Option<UserId>,
    pub email: String,
    pub password: String,
    pub company_id: CompanyId,
    pub role_id: RoleId,
    pub is_delete: bool
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserId(pub i32);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserEmail(pub String);

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserInfo {
    pub id: UserId,
    pub email: String,
    pub company_id: CompanyId,
    pub role_id: RoleId,
    pub is_delete: bool
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AuthInfo {
    pub email: String,
    pub password: String,
}

// User Model Access Control (UserMac)
pub struct UserMac;

pub trait UserActions {
    async fn create(store: Store, new_user: AuthInfo)
                    -> Result<UserInfo, Error>;
    async fn get_by_email(store: Store, user_email: &String)
                 -> Result<User, Error>;
    async fn get_by_id(store: Store, user_id: UserId)
                          -> Result<UserInfo, Error>;
    async fn list(store: Store, limit: Option<i32>, offset: i32)
                  -> Result<Vec<UserInfo>, Error>;
    async fn update_user(store: Store, user_update: UserInfo)
                         -> Result<UserInfo, Error>;
    async fn update_password(store: Store, user_update: AuthInfo)
                             -> Result<UserInfo, Error>;
    async fn set_role(store: Store, user_info: UserInfo, role_id: RoleId)
                       -> Result<UserInfo, Error>;
    async fn delete(store: Store, user_id: UserId)
                    -> Result<bool, Error>;
}
impl UserActions for UserMac {
    async fn create(store: Store, new_user: AuthInfo)
        -> Result<UserInfo, Error>
    {
        match store.create_user(new_user).await {
            Ok(user) => Ok(user),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(e)
            }
        }
    }

    async fn get_by_email(store: Store, user_email: &String)
                        -> Result<User, Error>
    {
        match store.get_user_by_email(user_email).await {
            Ok(user) => Ok(user),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(e)
            }
        }
    }

    async fn get_by_id(store: Store, user_id: UserId)
                       -> Result<UserInfo, Error>
    {
        match store.get_user_by_id(user_id).await {
            Ok(user) => Ok(user),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(e)
            }
        }
    }

    async fn list(store: Store, limit: Option<i32>, offset: i32)
                     -> Result<Vec<UserInfo>, Error>
    {
        match store.get_list_user(limit, offset).await {
            Ok(users) => Ok(users),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(e)
            }
        }
    }

    async fn update_user(store: Store, user_update: UserInfo)
                      -> Result<UserInfo, Error>
    {
        match store.update_user(user_update).await {
            Ok(user) => Ok(user),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(e)
            }
        }
    }

    async fn update_password(store: Store, user_update: AuthInfo)
                             -> Result<UserInfo, Error>
    {
        match store.update_password(user_update).await {
            Ok(user) => Ok(user),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(e)
            }
        }
    }

    async fn set_role(store: Store, user_info: UserInfo, role_id: RoleId)
                                 -> Result<UserInfo, Error>
    {
        match store.set_role(user_info, role_id).await {
            Ok(user) => Ok(user),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(e)
            }
        }
    }

    async fn delete(store: Store, user_id: UserId)
                           -> Result<bool, Error>
    {
        match store.delete_user_by_id(user_id).await {
            Ok(is_delete) => Ok(is_delete),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(e)
            }
        }
    }
}

// TEST
#[cfg(test)]
#[path = "../_tests/model_user.rs"]
mod tests;
