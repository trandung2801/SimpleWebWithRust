use std::collections::HashMap;
use std::sync::Arc;

use tracing::instrument;
use warp::http::StatusCode;

use crate::errors::Error;
use crate::models::company::{Company, CompanyId, NewCompany};
use crate::models::pagination::Pagination;
use crate::models::store_trait::StoreMethods;
use crate::services::jwt::Claims;
use crate::utils::convert_to_json::{Data, PayloadNoData, PayloadWithData};

// Handle for creating company
#[instrument(level = "info", skip(store))]
pub async fn create_company(
    store: Arc<dyn StoreMethods + Send + Sync>,
    claims: Claims,
    new_company: NewCompany,
) -> Result<impl warp::Reply, warp::Rejection> {
    //Check valid company
    if let Ok(_res) = store.get_company_by_email(&new_company.email).await {
        let payload = PayloadNoData {
            message: "Email company already exists".to_string(),
        };
        return Ok(warp::reply::with_status(
            warp::reply::json(&payload),
            StatusCode::BAD_REQUEST,
        ));
    }
    let res = store
        .create_company(new_company)
        .await
        .map_err(Error::from)?;
    let payload = PayloadWithData {
        message: "Success".to_string(),
        data: Data::Company(res),
    };
    Ok(warp::reply::with_status(
        warp::reply::json(&payload),
        StatusCode::CREATED,
    ))
}

// Handle for retrieving company by ID
#[instrument(level = "info", skip(store))]
pub async fn get_company(
    store: Arc<dyn StoreMethods + Send + Sync>,
    company_id: i32,
) -> Result<impl warp::Reply, warp::Rejection> {
    let res = store
        .get_company_by_id(CompanyId(company_id))
        .await
        .map_err(Error::from)?;
    let payload = PayloadWithData {
        message: "Success".to_string(),
        data: Data::Company(res),
    };
    Ok(warp::reply::with_status(
        warp::reply::json(&payload),
        StatusCode::OK,
    ))
}
// Handle for retrieving list companies based on query parameters
#[instrument(level = "info", skip(store))]
pub async fn get_list_company(
    store: Arc<dyn StoreMethods + Send + Sync>,
    params: HashMap<String, String>,
) -> Result<impl warp::Reply, warp::Rejection> {
    // Get pagination from query parameters
    let mut pagination = Pagination::default();

    if !params.is_empty() {
        pagination = Pagination::extract_pagination(params)?;
    }
    // Get list companies with pagination filters
    let res = store
        .get_list_company(pagination.limit, pagination.offset)
        .await
        .map_err(Error::from)?;
    let payload = PayloadWithData {
        message: "Success".to_string(),
        data: Data::ListCompany(res),
    };
    Ok(warp::reply::with_status(
        warp::reply::json(&payload),
        StatusCode::OK,
    ))
}

// Handler for updating company.
#[instrument(level = "info", skip(store))]
pub async fn update_company(
    store: Arc<dyn StoreMethods + Send + Sync>,
    claims: Claims,
    company: Company,
) -> Result<impl warp::Reply, warp::Rejection> {
    // Check valid company
    if let Ok(_res) = store.get_company_by_email(&company.email).await {
        let payload = PayloadNoData {
            message: "Email company already exists".to_string(),
        };
        return Ok(warp::reply::with_status(
            warp::reply::json(&payload),
            StatusCode::BAD_REQUEST,
        ));
    }
    let res = store.update_company(company).await.map_err(Error::from)?;
    let payload = PayloadWithData {
        message: "Success".to_string(),
        data: Data::Company(res),
    };
    Ok(warp::reply::with_status(
        warp::reply::json(&payload),
        StatusCode::OK,
    ))
}

// Handler for deleting company by ID.
#[instrument(level = "info", skip(store))]
pub async fn delete_company(
    store: Arc<dyn StoreMethods + Send + Sync>,
    claims: Claims,
    company: Company,
) -> Result<impl warp::Reply, warp::Rejection> {
    let _ = store
        .delete_company(company.id.unwrap())
        .await
        .map_err(Error::from)?;
    Ok(warp::reply::with_status(
        "Success".to_string(),
        StatusCode::OK,
    ))
}
