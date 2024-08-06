use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use crate::models::company::{Company, CompanyInfo};
use crate::models::job::{Job, JobInfo};
use crate::models::resume::{Resume, ResumeInfo};
use crate::models::role::{Role, RoleInfo};
use crate::models::user::UserInfo;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum Data {
    UserInfo(UserInfo),
    CompanyInfo(CompanyInfo),
    RoleInfo(RoleInfo),
    JobInfo(JobInfo),
    ResumeInfo(ResumeInfo),

    Company(Company),
    Role(Role),
    Job(Job),
    Resume(Resume),

    ListCompany(Vec<Company>),
    ListRole(Vec<Role>),
    ListJob(Vec<Job>),
    ListResume(Vec<Resume>),

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