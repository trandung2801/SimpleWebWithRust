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
use crate::models::company::{CompanyInfo, CompanyMac, CompanyActions, Company, CompanyId};
use crate::models::role::ADMIN_ROLE_ID;
use crate::models::store::Store;
use crate::models::user::{UserMac, UserActions};



pub async fn create_company(store: Store, claims: Claims, company_info: CompanyInfo)
                            -> Result<impl warp::Reply, warp::Rejection>
{
    //Authorization
    if(claims.role.0 != ADMIN_ROLE_ID) {
        let payload = PayloadNoData {
            status_code: StatusCode::UNAUTHORIZED,
            message: "Unauthorized".to_string(),
        };
        return Ok(warp::reply::json(&payload))
    }
    //valid company
    let new_email = company_info.email;
    match CompanyMac::get_by_email(store.clone(), &new_email).await {
        Ok(res) => {
            // let status_code = StatusCode::BAD_REQUEST;
            // let payload = json!({
            //     "statusCode": status_code,
            //     "message": "Email invalid",
            // });
            let payload = PayloadNoData {
                status_code: StatusCode::BAD_REQUEST,
                message: "Email company invalid".to_string(),
            };
            return Ok(warp::reply::json(&payload))
        }
        _ => ()
    }
    let new_company = CompanyInfo {
        email: new_email,
        name: company_info.name,
        address: company_info.address,
        description: company_info.description,
        is_delete: company_info.is_delete
    };
    match CompanyMac::create(store, new_company).await {
        Ok(res) =>
            {
                // let status_code = StatusCode::CREATED;
                // let payload = json!({
                //     "statusCode": status_code,
                //     "message": "Create company success",
                //     "data": res
                // });
                let payload = PayloadWithData {
                    status_code: StatusCode::CREATED,
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
                    status_code: StatusCode::OK,
                    message: "Get Company Success".to_string(),
                    data: Data::Company(res)
                };
                Ok(warp::reply::json(&payload))
            }
        Err(e) => Err(warp::reject()),
    }
}

pub async fn get_list_company(store: Store)
                                       -> Result<impl warp::Reply, warp::Rejection>
{
    match CompanyMac::list(store).await {
        Ok(res) =>
            {
                // let status_code = StatusCode::OK;
                // let payload = json!({
                //     "statusCode": status_code,
                //     "message": "get list companies success",
                //     "data": res
                // });
                let payload = PayloadWithData {
                    status_code: StatusCode::OK,
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
    //Authorization
    if(claims.role.0 != ADMIN_ROLE_ID) {
        let payload = PayloadNoData {
            status_code: StatusCode::UNAUTHORIZED,
            message: "Unauthorized".to_string(),
        };
        return Ok(warp::reply::json(&payload))
    }
    let email_update = company.email.clone();
    match CompanyMac::get_by_email(store.clone(), &email_update).await {
        Ok(res) => {
            // let status_code = StatusCode::BAD_REQUEST;
            // let payload = json!({
            //     "statusCode": status_code,
            //     "message": "Email invalid",
            // });
            let payload = PayloadNoData {
                status_code: StatusCode::BAD_REQUEST,
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
                    status_code: StatusCode::OK,
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
    //Authorization
    if(claims.role.0 != ADMIN_ROLE_ID) {
        let payload = PayloadNoData {
            status_code: StatusCode::UNAUTHORIZED,
            message: "Unauthorized".to_string(),
        };
        return Ok(warp::reply::json(&payload))
    }
    match CompanyMac::delete(store, company.id.unwrap()).await {
        Ok(_) =>
            {
                let status_code = StatusCode::OK;
                let payload = json!({
                    "statusCode": status_code,
                    "message": "delete user success",
                });
                Ok(warp::reply::json(&payload))
            }
        Err(e) => Err(warp::reject::custom(e)),
    }
}



