use serde::{Deserialize, Serialize};

pub const ADMIN_ROLE_ID: i32 = 1;
pub const USER_ROLE_ID: i32 = 2;
pub const HR_ROLE_ID: i32 = 3;
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Role {
    pub id: Option<RoleId>,
    pub role: String,
    pub is_delete: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct RoleId(pub i32);

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct RoleInfo {
    pub role: String,
    pub is_delete: bool,
}
