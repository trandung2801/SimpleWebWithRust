use serde::{Deserialize, Serialize};

use crate::models::company::CompanyId;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
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
