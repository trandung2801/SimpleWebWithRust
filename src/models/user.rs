use serde::{Deserialize, Serialize};
use handle_errors::Error;
use super::store::Store;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    pub id: Option<UserId>,
    pub email: String,
    pub password: String,
    pub company: String,
    // pub company: Option<CompanyId>,
    pub is_admin: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserId(pub i32);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserEmail(pub String);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CompanyId(pub i32);

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserInfo {
    pub id: i32,
    pub email: String,
    pub company: String,
    pub is_admin: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AuthInfo {
    pub email: String,
    pub password: String,
}

// User Model Access Control (UserMac)
pub struct UserMac;
impl UserMac {
    pub async fn create(store: Store, new_user: AuthInfo)
        -> Result<User, Error>
    {
        match store.create_user(new_user).await {
            Ok(user) => Ok(user),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(e)
            }
        }
    }

    pub async fn get(store: Store, user_email: &String)
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

    pub async fn list(store: Store)
                     -> Result<Vec<User>, Error>
    {
        match store.get_list_user().await {
            Ok(users) => Ok(users),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(e)
            }
        }
    }

    pub async fn update_user(store: Store, user_update: UserInfo)
                      -> Result<User, Error>
    {
        match store.update_user(user_update).await {
            Ok(user) => Ok(user),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(e)
            }
        }
    }

    pub async fn update_password(store: Store, user_update: AuthInfo)
                             -> Result<User, Error>
    {
        match store.update_password(user_update).await {
            Ok(user) => Ok(user),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(e)
            }
        }
    }

    pub async fn set_admin(store: Store, user_info: UserInfo)
                                 -> Result<User, Error>
    {
        match store.set_admin_role(user_info).await {
            Ok(user) => Ok(user),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(e)
            }
        }
    }

    pub async fn delete(store: Store, user_email: String)
                           -> Result<bool, Error>
    {
        match store.delete_user_by_email(user_email).await {
            Ok(is_delete) => Ok(is_delete),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(e)
            }
        }
    }
}
