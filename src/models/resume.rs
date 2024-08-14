use serde::{Deserialize, Serialize};
use crate::models::user::{UserId};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Resume {
    pub id: Option<ResumeId>,
    pub user_id: UserId,
    pub email: String,
    pub url: String,
    pub is_delete: bool
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResumeId(pub i32);

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewResume{
    pub user_id: UserId,
    pub email: String,
    pub url: String,
}

// async fn list_by_job_id(store: &Arc<dyn StoreMethods>, limit: Option<i32>, offset: i32, job_id: JobId)
//                         -> Result<Vec<Resume>, Error>
// {
//     match store.clone().get_list_resume_by_job_id(limit, offset, job_id).await {
//         Ok(map_resume_job) => {
//             let mut resume_list= Vec::new();
//             for e in map_resume_job {
//                 let resume =  store.clone().get_resume_by_id(e.resume_id).await?;
//                 resume_list.push(resume);
//             }
//             Ok(resume_list)
//         }
//         Err(e) => {
//             tracing::event!(tracing::Level::ERROR, "{:?}", e);
//             Err(e)
//         }
//     }
// }