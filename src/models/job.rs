use crate::models::company::CompanyId;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Job {
    pub id: Option<JobId>,
    pub job_name: String,
    pub company_id: CompanyId,
    pub location: String,
    pub quantity: i32,
    pub salary: i32,
    pub job_level: String,
    pub description: String,
    pub is_delete: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct JobId(pub i32);

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct NewJob {
    pub job_name: String,
    pub company_id: CompanyId,
    pub location: String,
    pub quantity: i32,
    pub salary: i32,
    pub job_level: String,
    pub description: String,
}
