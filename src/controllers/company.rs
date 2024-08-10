use std::collections::HashMap;
use tracing::{event, Level};
use warp::http::StatusCode;
use crate::middleware::convert_to_json::{Data, PayloadNoData, PayloadWithData};
use crate::middleware::jwt::{Claims};
use crate::models::company::{CompanyMac, CompanyActions, Company, CompanyId, NewCompany};
use crate::models::pagination::{Pagination, PaginationMethods};
use crate::models::store::Store;
use crate::models::user::{UserActions};

pub async fn create_company(store: Store, claims: Claims, new_company: NewCompany)
                            -> Result<impl warp::Reply, warp::Rejection>
{
    //valid company,
    let new_email = new_company.email;
    match CompanyMac::get_by_email(store.clone(), &new_email).await {
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
    match CompanyMac::create(store, company).await {
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

pub async fn get_company(store: Store, company_id: i32)
                                 -> Result<impl warp::Reply, warp::Rejection>
{
    event!(target: "backend", Level::INFO, "querying company");
    match CompanyMac::get_by_id(store, CompanyId(company_id)).await {
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

pub async fn get_list_company(store: Store, params: HashMap<String, String>)
                                       -> Result<impl warp::Reply, warp::Rejection>
{
    let mut pagination = Pagination::default();

    if !params.is_empty() {
        event!(Level::INFO, pagination = true);
        pagination = <Pagination as PaginationMethods>::extract_pagination(params)?;
    }
    match CompanyMac::list(store, pagination.limit, pagination.offset).await {
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


pub async fn update_company(store: Store, claims: Claims, company: Company)
                                    -> Result<impl warp::Reply, warp::Rejection>
{
    let email_update = company.email.clone();
    match CompanyMac::get_by_email(store.clone(), &email_update).await {
        Ok(res) => {
            let payload = PayloadNoData {
                message: "Email company invalid".to_string(),
            };
            return Ok(warp::reply::with_status(warp::reply::json(&payload), StatusCode::BAD_REQUEST))
        }
        _ => ()
    }
    match CompanyMac::update(store, company).await {
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

pub async fn delete_company(store: Store, claims: Claims, company: Company)
                               -> Result<impl warp::Reply, warp::Rejection>
{
    match CompanyMac::delete(store, company.id.unwrap()).await {
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





