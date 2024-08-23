use crate::models::company::CompanyId;
use crate::models::role::RoleId;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    pub id: Option<UserId>,
    pub email: String,
    pub password: String,
    pub company_id: CompanyId,
    pub role_id: RoleId,
    pub is_delete: bool,
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
    pub is_delete: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AuthInfo {
    pub email: String,
    pub password: String,
}
