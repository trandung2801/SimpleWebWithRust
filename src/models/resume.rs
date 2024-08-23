use crate::models::user::UserId;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Resume {
    pub id: Option<ResumeId>,
    pub user_id: UserId,
    pub email: String,
    pub url: String,
    pub is_delete: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResumeId(pub i32);

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewResume {
    pub user_id: UserId,
    pub email: String,
    pub url: String,
}
