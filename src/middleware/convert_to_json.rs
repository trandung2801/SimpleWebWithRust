use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use crate::models::company::{Company};
use crate::models::job::{Job};
use crate::models::resume::{Resume};
use crate::models::role::{Role};
use crate::models::user::UserInfo;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum Data {
    UserInfo(UserInfo),
    ListUserInfo(Vec<UserInfo>),

    Company(Company),
    Role(Role),
    Job(Job),
    Resume(Resume),

    ListCompany(Vec<Company>),
    ListRole(Vec<Role>),
    ListJob(Vec<Job>),
    ListResume(Vec<Resume>),



}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PayloadWithData {
    // pub status_code: StatusCode,
    pub message: String,
    pub data: Data
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PayloadNoData{
    // pub status_code: StatusCode,
    pub message: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PayloadForLogin{
    // pub status_code: StatusCode,
    pub message: String,
    pub access_token: String,
    pub data: Data
}