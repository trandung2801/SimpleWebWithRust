use serde::{Deserialize, Serialize};
use handle_errors::Error;
use crate::models::resume::{Resume, ResumeInfo};
use crate::models::store::Store;
use crate::models::store_impl_role::RoleStoreMethods;
use crate::models::user::{CompanyId, UserId};


pub const ADMIN_ROLE_ID: i32 = 1;
pub const USER_ROLE_ID: i32 = 2;
pub const HR_ROLE_ID: i32 = 3;
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Role {
    pub id: Option<RoleId>,
    pub role: String,
    pub is_delete: bool
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct RoleId(pub i32);

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct RoleInfo {
    pub role: String,
    pub is_delete: bool
}

pub struct RoleMac;

pub trait RoleActions {
    async fn create(store: Store, role_info: RoleInfo)
                    -> Result<Role, Error>;
    async fn get_by_id(store: Store, role_id: RoleId)
                       -> Result<Role, Error>;
    async fn list(store: Store)
                  -> Result<Vec<Role>, Error>;
    async fn update(store: Store, role_info: RoleInfo)
                    -> Result<Role, Error>;
    async fn delete(store: Store, role_id: RoleId)
                    -> Result<bool, Error>;
}

impl RoleActions for RoleMac {
    async fn create(store: Store, role_info: RoleInfo)
                        -> Result<Role, Error>
    {
        match store.create_role(role_info).await {
            Ok(role) => Ok(role),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(e)
            }
        }
    }

    async fn get_by_id(store: Store, role_id: RoleId)
                                -> Result<Role, Error>
    {
        match store.get_role_by_id(role_id).await {
            Ok(role) => Ok(role),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(e)
            }
        }
    }

    async fn list(store: Store)
                      -> Result<Vec<Role>, Error>
    {
        match store.get_list_roles().await {
            Ok(role_list) => Ok(role_list),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(e)
            }
        }
    }

    async fn update(store: Store, role_info: RoleInfo)
                        -> Result<Role, Error>
    {
        match store.update_role(role_info).await {
            Ok(role) => Ok(role),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(e)
            }
        }
    }

    async fn delete(store: Store, role_id: RoleId)
                        -> Result<bool, Error>
    {
        match store.delete_role(role_id).await {
            Ok(is_delete) => Ok(is_delete),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(e)
            }
        }
    }
}