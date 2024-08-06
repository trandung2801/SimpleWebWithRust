use serde::{Deserialize, Serialize};
use crate::models::store::Store;
use crate::models::user::{AuthInfo, UserId};
use handle_errors::Error;
use crate::models::store_impl_resume::ResumeStoreMethods;

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
pub struct ResumeInfo {
    pub user_id: UserId,
    pub email: String,
    pub url: String,
    pub is_delete: bool
}

pub struct ResumeMac;

pub trait ResumeActions {
    async fn create(store: Store, resume_info: ResumeInfo)
                    -> Result<Resume, Error>;
    async fn get_by_user_id(store: Store, user_id: UserId)
                            -> Result<Resume, Error>;
    async fn get_by_id(store: Store, resume_id: ResumeId)
                       -> Result<Resume, Error>;
    async fn list(store: Store)
                  -> Result<Vec<Resume>, Error>;
    async fn update(store: Store, resume_info: ResumeInfo)
                    -> Result<Resume, Error>;
    async fn delete(store: Store, resume_id: ResumeId)
                    -> Result<bool, Error>;
}

impl ResumeActions for ResumeMac {
    async fn create(store: Store, resume_info: ResumeInfo)
                        -> Result<Resume, Error>
    {
        match store.create_resume(resume_info).await {
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
            Ok(company) => Ok(company),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(e)
            }
        }
    }

    async fn list(store: Store)
                      -> Result<Vec<Resume>, Error>
    {
        match store.get_list_resumes().await {
            Ok(resume_list) => Ok(resume_list),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(e)
            }
        }
    }

    async fn update(store: Store, resume_info: ResumeInfo)
                        -> Result<Resume, Error>
    {
        match store.update_resume(resume_info).await {
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