use serde::{Deserialize, Serialize};
use handle_errors::Error;
use crate::models::company::CompanyId;
use crate::models::role::{Role, RoleId, RoleInfo};
use crate::models::store::Store;
use crate::models::store_impl_job::JobStoreMethods;

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
    pub is_delete: bool
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
pub struct JobMac;

pub trait JobActions {
    async fn create(store: Store, new_job: NewJob)
                    -> Result<Job, Error>;
    async fn get_by_id(store: Store, job_id: JobId)
                       -> Result<Job, Error>;
    async fn list(store: Store, limit: Option<i32>, offset: i32)
                  -> Result<Vec<Job>, Error>;
    async fn update(store: Store, job: Job)
                    -> Result<Job, Error>;
    async fn delete(store: Store, job_id: JobId)
                    -> Result<bool, Error>;
}

impl JobActions for JobMac {
    async fn create(store: Store, new_job: NewJob) -> Result<Job, Error>
    {
        match store.create_job(new_job).await {
            Ok(job) => Ok(job),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(e)
            }
        }
    }
    async fn get_by_id(store: Store, job_id: JobId) -> Result<Job, Error>
    {
        match store.get_job_by_id(job_id).await {
            Ok(job) => Ok(job),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(e)
            }
        }
    }
    async fn list(store: Store, limit: Option<i32>, offset: i32) -> Result<Vec<Job>, Error>
    {
        match store.get_list_job(limit, offset).await {
            Ok(job_list) => Ok(job_list),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(e)
            }
        }
    }
    async fn update(store: Store, job: Job) -> Result<Job, Error> {
        match store.update_job(job).await {
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(e)
            }
            Ok(job) => Ok(job),
        }
    }
    async fn delete(store: Store, job_id: JobId) -> Result<bool, Error> {
        match store.delete_job(job_id).await {
            Ok(is_delete) => Ok(is_delete),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(e)
            }
        }
    }
}

// TEST
#[cfg(test)]
#[path = "../_tests/model_job.rs"]
mod tests;