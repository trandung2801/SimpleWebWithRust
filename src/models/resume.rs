use serde::{Deserialize, Serialize};
use crate::models::store::Store;
use crate::models::user::{AuthInfo, UserId};
use handle_errors::Error;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Resume {
    pub id: Option<ResumeId>,
    pub user_id: i32,
    pub email: String,
    pub url: String,
    pub status: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResumeId(pub i32);

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ResumeInfo {
    pub user_id: i32,
    pub email: String,
    pub url: String,
    pub status: bool,
}

pub struct CompanyMac;

impl CompanyMac {
    pub async fn create(store: Store, resume_info: ResumeInfo)
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

    // pub async fn get(store: Store, company_email: &String)
    //                  -> Result<Company, Error>
    // {
    //     match store.get_company_by_email(company_email).await {
    //         Ok(company) => Ok(company),
    //         Err(e) => {
    //             tracing::event!(tracing::Level::ERROR, "{:?}", e);
    //             Err(e)
    //         }
    //     }
    // }
    //
    // pub async fn list(store: Store)
    //                   -> Result<Vec<Company>, Error>
    // {
    //     match store.get_list_company().await {
    //         Ok(company_list) => Ok(company_list),
    //         Err(e) => {
    //             tracing::event!(tracing::Level::ERROR, "{:?}", e);
    //             Err(e)
    //         }
    //     }
    // }
    //
    //
    // pub async fn update(store: Store, company_info: CompanyInfo)
    //                     -> Result<Company, Error>
    // {
    //     match store.update_company(company_info).await {
    //         Ok(company) => Ok(company),
    //         Err(e) => {
    //             tracing::event!(tracing::Level::ERROR, "{:?}", e);
    //             Err(e)
    //         }
    //     }
    // }
    //
    // pub async fn delete(store: Store, company_info: CompanyInfo)
    //                     -> Result<bool, Error>
    // {
    //     match store.delete_company_by_email(company_info).await {
    //         Ok(is_delete) => Ok(is_delete),
    //         Err(e) => {
    //             tracing::event!(tracing::Level::ERROR, "{:?}", e);
    //             Err(e)
    //         }
    //     }
    // }
}