use serde::{Deserialize, Serialize};

use crate::models::company::CompanyId;
use crate::models::role::RoleId;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub id: Option<UserId>,
    pub email: String,
    pub hash_password: String,
    pub company_id: CompanyId,
    pub role_id: RoleId,
    pub is_delete: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserId(pub i32);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserEmail(pub String);

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct UserInfo {
    pub id: UserId,
    pub email: String,
    pub company_id: CompanyId,
    pub role_id: RoleId,
    pub is_delete: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AuthInfo {
    pub email: String,
    pub hash_password: String,
}
