use serde::{Deserialize, Serialize};
use crate::models::job::JobId;
use crate::models::resume::ResumeId;


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
