use serde::{Deserialize, Serialize};
use crate::models::store::{MapResumeJobMethods, Store};
use crate::models::user::{AuthInfo, UserId};
use handle_errors::Error;
use crate::models::job::JobId;
use crate::models::store_impl_resume::ResumeStoreMethods;
use crate::models::map_resume_job::MapResumeJobActions;

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


pub struct ResumeMac;

pub trait ResumeActions {
    async fn create(store: Store, new_resume: NewResume)
                    -> Result<Resume, Error>;
    async fn get_by_user_id(store: Store, user_id: UserId)
                            -> Result<Resume, Error>;
    async fn get_by_id(store: Store, resume_id: ResumeId)
                       -> Result<Resume, Error>;
    async fn list_by_user_id(store: Store, limit: Option<i32>, offset: i32, user_id: UserId)
                  -> Result<Vec<Resume>, Error>;
    async fn list_by_job_id(store: Store, limit: Option<i32>, offset: i32, job_id: JobId)
                             -> Result<Vec<Resume>, Error>;
    async fn update(store: Store, resume: Resume)
                    -> Result<Resume, Error>;
    async fn delete(store: Store, resume_id: ResumeId)
                    -> Result<bool, Error>;
}

impl ResumeActions for ResumeMac {
    async fn create(store: Store, new_resume: NewResume)
                        -> Result<Resume, Error>
    {
        match store.create_resume(new_resume).await {
            Ok(new_resume) => Ok(new_resume),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(e)
            }
        }
    }

    async fn get_by_user_id(store: Store, user_id: UserId)
                     -> Result<Resume, Error>
    {
        match store.get_resume_by_user_id(user_id).await {
            Ok(company) => Ok(company),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(e)
            }
        }
    }

    async fn get_by_id(store: Store, resume_id: ResumeId)
                                -> Result<Resume, Error>
    {
        match store.get_resume_by_id(resume_id).await {
            Ok(resume) => Ok(resume),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(e)
            }
        }
    }

    async fn list_by_user_id(store: Store, limit: Option<i32>, offset: i32, user_id: UserId)
                      -> Result<Vec<Resume>, Error>
    {
        match store.get_list_resume_by_user_id(limit, offset, user_id).await {
            Ok(resume_list) => Ok(resume_list),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(e)
            }
        }
    }

    async fn list_by_job_id(store: Store, limit: Option<i32>, offset: i32, job_id: JobId)
                            -> Result<Vec<Resume>, Error>
    {
        match store.clone().get_list_resume_by_job_id(limit, offset, job_id).await {
            Ok(map_resume_job) => {
                let mut resume_list= Vec::new();
                for e in map_resume_job {
                    let resume =  store.clone().get_resume_by_id(e.resume_id).await?;
                    resume_list.push(resume);
                }
                Ok(resume_list)
            }
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(e)
            }
        }
    }

    async fn update(store: Store, resume: Resume)
                        -> Result<Resume, Error>
    {
        match store.update_resume(resume).await {
            Ok(resume) => Ok(resume),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(e)
            }
        }
    }

    async fn delete(store: Store, resume_id: ResumeId)
                        -> Result<bool, Error>
    {
        match store.delete_resume(resume_id).await {
            Ok(is_delete) => Ok(is_delete),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(e)
            }
        }
    }
}