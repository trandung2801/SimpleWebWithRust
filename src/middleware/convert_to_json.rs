use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use crate::models::company::CompanyInfo;
use crate::models::job::JobInfo;
use crate::models::resume::ResumeInfo;
use crate::models::role::RoleInfo;
use crate::models::user::UserInfo;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum Data {
    UserInfo(UserInfo),
    CompanyInfo(CompanyInfo),
    RoleInfo(RoleInfo),
    JobInfo(JobInfo),
    ResumeInfo(ResumeInfo),

    ListUserInfo(Vec<UserInfo>),
    ListCompanyInfo(Vec<CompanyInfo>),
    ListRoleInfo(Vec<RoleInfo>),
    ListJobInfo(Vec<JobInfo>),
    ListResumeInfo(Vec<ResumeInfo>),
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PayloadWithData {
    pub status_code: StatusCode,
    pub message: String,
    pub data: Data
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PayloadNoData{
    pub status_code: StatusCode,
    pub message: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PayloadForLogin{
    pub status_code: StatusCode,
    pub message: String,
    pub access_token: String,
    pub data: Data
}