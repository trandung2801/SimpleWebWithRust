use std::collections::HashMap;
use std::sync::Arc;
use argon2::Config;
use rand::Rng;
use serde_json::json;
use tokio::join;
use tracing::{event, instrument, Level};
use handle_errors::Error;
use warp::http::StatusCode;
use crate::middleware::convert_to_json::{Data, PayloadNoData, PayloadWithData};
use crate::middleware::jwt::{Jwt, Claims};
use crate::models::company::{CompanyMac, CompanyActions, Company, CompanyId, NewCompany};
use crate::models::pagination::{Pagination, PaginationMethods};
use crate::models::role::ADMIN_ROLE_ID;
use crate::models::store::Store;
use crate::models::user::{UserMac, UserActions};

pub async fn create_company(store: Store, claims: Claims, new_company: NewCompany)
                            -> Result<impl warp::Reply, warp::Rejection>
{
    //valid company,
    let new_email = new_company.email;
    match CompanyMac::get_by_email(store.clone(), &new_email).await {
        Ok(res) => {
            // let status_code = StatusCode::BAD_REQUEST;
            // let payload = json!({
            //     "statusCode": status_code,
            //     "message": "Email invalid",
            // });
            let payload = PayloadNoData {
                // status_code: StatusCode::BAD_REQUEST,
                message: "Email company invalid".to_string(),
            };
            return Ok(warp::reply::json(&payload))
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
                // let status_code = StatusCode::CREATED;
                // let payload = json!({
                //     "statusCode": status_code,
                //     "message": "Create company success",
                //     "data": res
                // });
                let payload = PayloadWithData {
                    // status_code: StatusCode::CREATED,
                    message: "Create Company Success".to_string(),
                    data: Data::Company(res)
                };
                Ok(warp::reply::json(&payload))
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
                // let status_code = StatusCode::OK;
                // let payload = json!({
                //     "statusCode": status_code,
                //     "message": "get company success",
                //      "data": res,
                // });
                let payload = PayloadWithData {
                    // status_code: StatusCode::OK,
                    message: "Get Company Success".to_string(),
                    data: Data::Company(res)
                };
                Ok(warp::reply::json(&payload))
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
                // let status_code = StatusCode::OK;
                // let payload = json!({
                //     "statusCode": status_code,
                //     "message": "get list companies success",
                //     "data": res
                // });
                let payload = PayloadWithData {
                    // status_code: StatusCode::OK,
                    message: "Get List Company Success".to_string(),
                    data: Data::ListCompany(res)
                };
                Ok(warp::reply::json(&payload))
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
            // let status_code = StatusCode::BAD_REQUEST;
            // let payload = json!({
            //     "statusCode": status_code,
            //     "message": "Email invalid",
            // });
            let payload = PayloadNoData {
                // status_code: StatusCode::BAD_REQUEST,
                message: "Email company invalid".to_string(),
            };
            return Ok(warp::reply::json(&payload))
        }
        _ => ()
    }
    match CompanyMac::update(store, company).await {
        Ok(res) =>
            {
                // let status_code = StatusCode::OK;
                // let payload = json!({
                //     "statusCode": status_code,
                //     "message": "update company success",
                //     "data": res
                // });
                let payload = PayloadWithData {
                    // status_code: StatusCode::OK,
                    message: "Update Company Success".to_string(),
                    data: Data::Company(res)
                };
                Ok(warp::reply::json(&payload))
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
                // let status_code = StatusCode::OK;
                // let payload = json!({
                //     "statusCode": status_code,
                //     "message": "Delete Company success",
                // });
                let payload = PayloadNoData {
                    // status_code: StatusCode::OK,
                    message: "Delete Company success".to_string()
                };
                Ok(warp::reply::json(&payload))
            }
        Err(e) => Err(warp::reject::custom(e)),
    }
}



