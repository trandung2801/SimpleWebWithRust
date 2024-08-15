use std::collections::HashMap;
use std::sync::Arc;
use warp::http::StatusCode;
use tracing::{event, instrument, Level};
use crate::service::convert_to_json::{Data, PayloadNoData, PayloadWithData};
use crate::middleware::jwt::{Claims};
use crate::models::company::{Company, CompanyId, NewCompany};
use crate::models::pagination::{Pagination, PaginationMethods};
use crate::models::store_trait::StoreMethods;

// Handle for create company
//
// This function adds a new company to the system. It takes company information to
// be created and a reference to the StoreMethods trait object for company. It
// returns a success response with status 200 if company is created successfully
#[instrument(level = "info", skip(store))]
pub async fn create_company(store: Arc<dyn StoreMethods + Send + Sync>, claims: Claims, new_company: NewCompany)
                            -> Result<impl warp::Reply, warp::Rejection>
{
    //Check valid company
    let new_email = new_company.email;
    match store.get_company_by_email(&new_email).await {
        Ok(_res) => {
            let payload = PayloadNoData {
                message:"Email company already exists".to_string(),
            };
            return Ok(warp::reply::with_status(
                warp::reply::json(&payload),
                StatusCode::BAD_REQUEST))
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

// Handle for retrieving company by ID
//
// This function retrieves a company with the specified ID from the system. It takes
// the ID from query parameters. It returns a JSON response containing the company.
#[instrument(level = "info", skip(store))]
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
        Err(e) => Err(warp::reject::custom(e)),
    }
}
// Handle for retrieving list companies based on query parameters
//
// This function retrieves list companies based on the provided query parameters. It takes a HashMap
// containing the query parameters and a reference to the StoreMethod trait object for company.
// It returns a JSON response containing the list of companies.
#[instrument(level = "info", skip(store))]
pub async fn get_list_company(store: Arc<dyn StoreMethods + Send + Sync>, params: HashMap<String, String>)
                                       -> Result<impl warp::Reply, warp::Rejection>
{
    // Get pagination from query parameters
    let mut pagination = Pagination::default();

    if !params.is_empty() {
        event!(Level::INFO, pagination = true);
        pagination = <Pagination as PaginationMethods>::extract_pagination(params)?;
    }
    // Get list companies with pagination filters
    match store.get_list_company(pagination.limit, pagination.offset).await {
        Ok(res) =>
            {
                let payload = PayloadWithData {
                    message: "Get List Company Success".to_string(),
                    data: Data::ListCompany(res)
                };
                Ok(warp::reply::with_status(warp::reply::json(&payload), StatusCode::OK))
            }
        Err(e) => Err(warp::reject::custom(e)),
    }
}

// Handler for updating company.
//
// This function updates company. It takes info of the company to be updated
// from Company and a reference to the StoreMethod trait object for company.
// It returns a success response with status code 200 if the company update successfully
#[instrument(level = "info", skip(store))]
pub async fn update_company(store: Arc<dyn StoreMethods + Send + Sync>, claims: Claims, company: Company)
                                    -> Result<impl warp::Reply, warp::Rejection>
{
    // Check valid company
    let email_update = company.email.clone();
    match store.get_company_by_email(&email_update).await {
        Ok(_res) => {
            let payload = PayloadNoData {
                message: "Email company already exists".to_string(),
            };
            return Ok(warp::reply::with_status(
                warp::reply::json(&payload),
                StatusCode::BAD_REQUEST))
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

// Handler for deleting company by ID.
//
// This function deletes company with the specified ID from the system. It takes the ID of
// the company to be deleted from Company and a reference to the StoreMethod trait object. It
// returns a success response with status code 200 if the company is deleted successfully.
#[instrument(level = "info", skip(store))]
pub async fn delete_company(store: Arc<dyn StoreMethods + Send + Sync>, claims: Claims, company: Company)
                               -> Result<impl warp::Reply, warp::Rejection>
{
    match store.delete_company(company.id.unwrap()).await {
        Ok(_) =>
            {
                Ok(warp::reply::with_status("Delete Company Success".to_string(), StatusCode::OK))
            }
        Err(e) => Err(warp::reject::custom(e)),
    }
}

#[cfg(test)]
#[path = "../tests/route_resume.rs"]
mod route_resume_tests;





