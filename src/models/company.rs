use std::sync::Arc;
use serde::{Deserialize, Serialize};
use handle_errors::Error;
use crate::models::store::{Store, StoreMethods};


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Company {
    pub id: Option<CompanyId>,
    pub name: String,
    pub email: String,
    pub address: String,
    pub description: String,
    pub is_delete: bool
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CompanyId(pub i32);

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewCompany {
    pub email: String,
    pub name: String,
    pub address: String,
    pub description: String,
}

pub struct CompanyMac;

pub trait CompanyActions {
    async fn create(store: &Arc<dyn StoreMethods>, new_company: NewCompany)
        -> Result<Company, Error>;
    async fn get_by_email(store: &Arc<dyn StoreMethods>, company_email: &String)
                 -> Result<Company, Error>;
    async fn get_by_id(store: &Arc<dyn StoreMethods>, company_id: CompanyId)
                          -> Result<Company, Error>;
    async fn list(store: &Arc<dyn StoreMethods>, limit: Option<i32>, offset: i32)
                  -> Result<Vec<Company>, Error>;
    async fn update(store: &Arc<dyn StoreMethods>, company: Company)
                    -> Result<Company, Error>;
    async fn delete(store: &Arc<dyn StoreMethods>, company_id: CompanyId)
                    -> Result<bool, Error>;
}

impl CompanyActions for CompanyMac {
    async fn create(store: &Arc<dyn StoreMethods>, new_company: NewCompany)
                        -> Result<Company, Error>
    {
        match store.create_company(new_company).await {
            Ok(new_company) => Ok(new_company),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(e)
            }
        }
    }

    async fn get_by_email(store: &Arc<dyn StoreMethods>, company_email: &String)
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

    async fn get_by_id(store: &Arc<dyn StoreMethods>, company_id: CompanyId)
        -> Result<Company, Error>
    {
        match store.get_company_by_id(company_id).await {
            Ok(company) => Ok(company),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(e)
            }
        }
    }

    async fn list(store: &Arc<dyn StoreMethods>, limit: Option<i32>, offset: i32)
                        -> Result<Vec<Company>, Error>
    {
        match store.get_list_company(limit, offset).await {
            Ok(company_list) => Ok(company_list),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(e)
            }
        }
    }

    async fn update(store: &Arc<dyn StoreMethods>, company: Company)
                        -> Result<Company, Error>
    {
        match store.update_company(company).await {
            Ok(company) => Ok(company),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(e)
            }
        }
    }

    async fn delete(store: &Arc<dyn StoreMethods>, company_id: CompanyId)
                        -> Result<bool, Error>
    {
        match store.delete_company(company_id).await {
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
#[path = "../tests/model_company.rs"]
mod model_company_tests;