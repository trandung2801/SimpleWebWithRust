use std::collections::HashMap;
use std::ffi::CStr;
use std::sync::Arc;
use warp::http::StatusCode;
use crate::middleware::convert_to_json::{Data, PayloadNoData, PayloadWithData};
use crate::middleware::jwt::{Claims};
use crate::models::company::{Company, CompanyId, NewCompany};
use crate::models::pagination::{Pagination, PaginationMethods};
use crate::models::store_trait::StoreMethods;
use tracing::instrument;

#[instrument(level = "info")]
pub async fn create_company(store: Arc<dyn StoreMethods + Send + Sync>, claims: Claims, new_company: NewCompany)
                            -> Result<impl warp::Reply, warp::Rejection>
{
    //valid company,
    let new_email = new_company.email;
    match store.get_company_by_email(&new_email).await {
        Ok(res) => {
            let payload = PayloadNoData {
                message: "Email company invalid".to_string(),
            };
            return Ok(warp::reply::with_status(warp::reply::json(&payload), StatusCode::BAD_REQUEST))
        }
        _ => ()
    }
    let company = NewCompany {
        email: new_email,
        name: new_company.name,
        address: new_company.address,
        description: new_company.description,
    };
    match store.create_company(company).await {
        Ok(res) =>
            {
                let payload = PayloadWithData {
                    message: "Create Company Success".to_string(),
                    data: Data::Company(res)
                };
                Ok(warp::reply::with_status(warp::reply::json(&payload), StatusCode::CREATED))
            }
        Err(e) => Err(warp::reject::custom(e)),
    }
}

#[instrument(level = "info")]
pub async fn get_company(store: Arc<dyn StoreMethods + Send + Sync>, company_id: i32)
                                 -> Result<impl warp::Reply, warp::Rejection>
{
    match store.get_company_by_id(CompanyId(company_id)).await {
        Ok(res) =>
            {
                let payload = PayloadWithData {
                    message: "Get Company Success".to_string(),
                    data: Data::Company(res)
                };
                Ok(warp::reply::with_status(warp::reply::json(&payload), StatusCode::OK))
            }
        Err(e) => Err(warp::reject()),
    }
}

#[instrument(level = "info")]
pub async fn get_list_company(store: Arc<dyn StoreMethods + Send + Sync>, params: HashMap<String, String>)
                                       -> Result<impl warp::Reply, warp::Rejection>
{
    let mut pagination = Pagination::default();

    if !params.is_empty() {
        pagination = <Pagination as PaginationMethods>::extract_pagination(params)?;
    }
    match store.get_list_company(pagination.limit, pagination.offset).await {
        Ok(res) =>
            {
                let payload = PayloadWithData {
                    message: "Get List Company Success".to_string(),
                    data: Data::ListCompany(res)
                };
                Ok(warp::reply::with_status(warp::reply::json(&payload), StatusCode::OK))
            }
        Err(e) => Err(warp::reject()),
    }
}

#[instrument(level = "info")]
pub async fn update_company(store: Arc<dyn StoreMethods + Send + Sync>, claims: Claims, company: Company)
                                    -> Result<impl warp::Reply, warp::Rejection>
{
    let email_update = company.email.clone();
    match store.get_company_by_email(&email_update).await {
        Ok(res) => {
            let payload = PayloadNoData {
                message: "Email company invalid".to_string(),
            };
            return Ok(warp::reply::with_status(warp::reply::json(&payload), StatusCode::BAD_REQUEST))
        }
        _ => ()
    }
    match store.update_company(company).await {
        Ok(res) =>
            {
                let payload = PayloadWithData {
                    message: "Update Company Success".to_string(),
                    data: Data::Company(res)
                };
                Ok(warp::reply::with_status(warp::reply::json(&payload), StatusCode::OK))
            }
        Err(e) => Err(warp::reject::custom(e)),
    }
}

#[instrument(level = "info")]
pub async fn delete_company(store: Arc<dyn StoreMethods + Send + Sync>, claims: Claims, company: Company)
                               -> Result<impl warp::Reply, warp::Rejection>
{
    match store.delete_company(company.id.unwrap()).await {
        Ok(_) =>
            {
                let payload = PayloadNoData {
                    message: "Delete Company success".to_string()
                };
                Ok(warp::reply::with_status(warp::reply::json(&payload), StatusCode::OK))
            }
        Err(e) => Err(warp::reject::custom(e)),
    }
}

#[cfg(test)]
#[path = "../tests/route_resume.rs"]
mod route_resume_tests;





