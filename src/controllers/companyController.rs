use std::collections::HashMap;
use std::sync::Arc;
use argon2::Config;
use rand::Rng;
use serde_json::json;
use tokio::join;
use tracing::{event, instrument, Level};
use handle_errors::Error;
use warp::http::StatusCode;
use crate::middleware::jwt::{Jwt, Claims};
use crate::models::company::{CompanyInfo, CompanyMac};
use crate::models::store::Store;

// async fn valid_user(claims: Claims, company_info: CompanyInfo) -> bool{
//
// }

pub async fn create_company(store: Store, claims: Claims, company_info: CompanyInfo)
                                 -> Result<impl warp::Reply, warp::Rejection>
{
    // valid_user(claims, company_info.clone());
    let new_email = company_info.email;
    match CompanyMac::get(store.clone(), &new_email).await {
        Ok(res) => {
            let payload = json!({
                "statusCode": 201,
                "message": "Email invalid",
            });
            return Ok(warp::reply::json(&payload))
        }
        _ => ()
    }
    let new_company = CompanyInfo {
        email: new_email,
        name: company_info.name,
        address: company_info.address,
        description: company_info.description
    };
    match CompanyMac::create(store, new_company).await {
        Ok(res) =>
            {
                let payload = json!({
                    "statusCode": 201,
                    "message": "Create company success",
                    "data": res
                });
                Ok(warp::reply::json(&payload))
            }
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn get_company(store: Store, company_email: String)
                                 -> Result<impl warp::Reply, warp::Rejection>
{
    event!(target: "backend", Level::INFO, "querying company");
    match CompanyMac::get(store, &company_email).await {
        Ok(company) =>
            {
                let payload = json!({
                    "statusCode": 201,
                    "message": "get company success",
                     "data": company,
                });
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
                let payload = json!({
                    "statusCode": 201,
                    "message": "get list companies success",
                    "data": res
                });
                Ok(warp::reply::json(&payload))
            }
        Err(e) => Err(warp::reject()),
    }
}


pub async fn update_company(store: Store, claims: Claims, company_update: CompanyInfo)
                                    -> Result<impl warp::Reply, warp::Rejection>
{
    match CompanyMac::update(store, company_update).await {
        Ok(res) =>
            {
                let payload = json!({
                    "statusCode": 201,
                    "message": "update company success",
                    "data": res
                });
                Ok(warp::reply::json(&payload))
            }
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn delete_company(store: Store, claims: Claims, user_delete: CompanyInfo)
                               -> Result<impl warp::Reply, warp::Rejection>
{
    match CompanyMac::delete(store, user_delete).await {
        Ok(_) =>
            {
                let payload = json!({
                    "statusCode": 201,
                    "message": "delete user success",
                });
                Ok(warp::reply::json(&payload))
            }
        Err(e) => Err(warp::reject::custom(e)),
    }
}



