use serde::{Deserialize, Serialize};
use handle_errors::Error;
use crate::models::job::JobId;
use crate::models::resume::ResumeId;
use crate::models::store::{MapResumeJobMethods, Store};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MapResumeJob {
    pub id: Option<MapResumeJobId>,
    pub resume_id: ResumeId,
    pub job_id: JobId
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewMapResumeJob {
    pub resume_id: ResumeId,
    pub job_id: JobId
}


#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MapResumeJobId(pub i32);

pub struct MapResumeJobMac;
pub trait MapResumeJobActions{
    async fn create(store: Store, new_map_resume_job: NewMapResumeJob) -> Result<MapResumeJob, Error>;
    async fn list_job_by_resume(store: Store, resume_id: ResumeId) -> Result<Vec<MapResumeJob>, Error>;
    async fn list_resume_by_job(store: Store, job_id: JobId) -> Result<Vec<MapResumeJob>, Error>;
}

impl MapResumeJobActions for MapResumeJobMac {
    async fn create(store: Store, new_map_resume_job: NewMapResumeJob) -> Result<MapResumeJob, Error>
    {
        match store.create_map_job_resume(new_map_resume_job).await
        {
            Ok(map) => Ok(map),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(e)
            }
        }
    }
    async fn list_job_by_resume(store: Store, resume_id: ResumeId) -> Result<Vec<MapResumeJob>, Error>
    {
        match store.get_list_job_by_resume(resume_id).await
        {
            Ok(list_map) => Ok(list_map),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(e)
            }
        }
    }
    async fn list_resume_by_job(store: Store, job_id: JobId) -> Result<Vec<MapResumeJob>, Error>
    {
        match store.get_list_resume_by_job(job_id).await
        {
            Ok(list_map) => Ok(list_map),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(e)
            }
        }
    }
}