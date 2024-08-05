use serde::{Deserialize, Serialize};
use crate::models::store::Store;
use crate::models::user::AuthInfo;
use handle_errors::Error;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Company {
    pub id: Option<CompanyId>,
    pub name: String,
    pub email: String,
    pub address: String,
    pub description: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CompanyId(pub i32);

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CompanyInfo {
    pub email: String,
    pub name: String,
    pub address: String,
    pub description: String,
}

pub struct CompanyMac;

impl CompanyMac {
    pub async fn create(store: Store, company_info: CompanyInfo)
                        -> Result<Company, Error>
    {
        match store.create_company(company_info).await {
            Ok(new_company) => Ok(new_company),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(e)
            }
        }
    }

    pub async fn get(store: Store, company_email: &String)
                        -> Result<Company, Error>
    {
        match store.get_company_by_email(company_email).await {
            Ok(company) => Ok(company),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(e)
            }
        }
    }

    pub async fn list(store: Store)
                        -> Result<Vec<Company>, Error>
    {
        match store.get_list_company().await {
            Ok(company_list) => Ok(company_list),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(e)
            }
        }
    }


    pub async fn update(store: Store, company_info: CompanyInfo)
                        -> Result<Company, Error>
    {
        match store.update_company(company_info).await {
            Ok(company) => Ok(company),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(e)
            }
        }
    }

    pub async fn delete(store: Store, company_info: CompanyInfo)
                        -> Result<bool, Error>
    {
        match store.delete_company_by_email(company_info).await {
            Ok(is_delete) => Ok(is_delete),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(e)
            }
        }
    }
}