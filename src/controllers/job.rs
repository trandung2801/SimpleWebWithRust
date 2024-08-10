use std::collections::HashMap;
use reqwest::StatusCode;
use tracing::{event, Level};
use crate::middleware::convert_to_json::{Data, PayloadNoData, PayloadWithData};
use crate::middleware::jwt::Claims;
use crate::models::job::{JobMac, NewJob, JobActions, JobId, Job};
use crate::models::store::Store;
use crate::models::user::{UserMac, UserActions};
use crate::models::pagination::{Pagination, PaginationMethods};


pub async fn create_job(store: Store, claims: Claims, new_job: NewJob)
                        -> Result<impl warp::Reply, warp::Rejection>
{
    match UserMac::get_by_id(store.clone(), claims.id).await {
        Ok(res) => {
            if res.company_id != new_job.clone().company_id {
                let payload = PayloadNoData {
                    message: "Can't update".to_string(),
                };
                return Ok(warp::reply::with_status(warp::reply::json(&payload), StatusCode::BAD_REQUEST))
            }
        }
        _ => ()
    }

    let job = NewJob {
        job_name: new_job.job_name,
        company_id: new_job.company_id,
        location: new_job.location,
        quantity: new_job.quantity,
        salary: new_job.salary,
        job_level: new_job.job_level,
        description: new_job.description
    };
    match JobMac::create(store, job).await {
        Ok(res) =>
            {
                let payload = PayloadWithData {
                    message: "Created Job Success".to_string(),
                    data: Data::Job(res)
                };
                Ok(warp::reply::with_status(warp::reply::json(&payload), StatusCode::CREATED))
            }
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn get_job(store: Store, job_id: i32)
                        -> Result<impl warp::Reply, warp::Rejection>
{
    match JobMac::get_by_id(store, JobId(job_id)).await {
        Ok(res) =>
            {
                let payload = PayloadWithData {
                    message: "Get Job Success".to_string(),
                    data: Data::Job(res)
                };
                Ok(warp::reply::with_status(warp::reply::json(&payload), StatusCode::OK))
            }
        Err(e) => Err(warp::reject()),
    }
}

pub async fn get_list_job(store: Store, params: HashMap<String, String>)
                                        -> Result<impl warp::Reply, warp::Rejection>
{
    let mut pagination = Pagination::default();

    if !params.is_empty() {
        event!(Level::INFO, pagination = true);
        pagination = <Pagination as PaginationMethods>::extract_pagination(params)?;
    }
    match JobMac::list(store, pagination.limit, pagination.offset).await {
        Ok(res) =>
            {
                let payload = PayloadWithData {
                    message: "Get List Job Success".to_string(),
                    data: Data::ListJob(res)
                };
                Ok(warp::reply::with_status(warp::reply::json(&payload), StatusCode::OK))
            }
        Err(e) => Err(warp::reject()),
    }
}

pub async fn update_job(store: Store, claims: Claims, job: Job)
                           -> Result<impl warp::Reply, warp::Rejection>
{
    match UserMac::get_by_id(store.clone(), claims.id).await {
        Ok(res) => {
            if res.company_id != job.clone().company_id {
                let payload = PayloadNoData {
                    message: "Can't update".to_string(),
                };
                return Ok(warp::reply::with_status(warp::reply::json(&payload), StatusCode::BAD_REQUEST))
            }
        }
        _ => ()
    }
    match JobMac::update(store, job).await {
        Ok(res) =>
            {
                let payload = PayloadWithData {
                    message: "Update Job Success".to_string(),
                    data: Data::Job(res)
                };
                Ok(warp::reply::with_status(warp::reply::json(&payload), StatusCode::OK))
            }
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn delete_job(store: Store, claims: Claims, job: Job)
                           -> Result<impl warp::Reply, warp::Rejection>
{
    match UserMac::get_by_id(store.clone(), claims.id).await {
        Ok(res) => {
            if res.company_id != job.clone().company_id {
                let payload = PayloadNoData {
                    message: "Can't update".to_string(),
                };
                return Ok(warp::reply::with_status(warp::reply::json(&payload), StatusCode::BAD_REQUEST))
            }
        }
        _ => ()
    }
    match JobMac::delete(store, job.id.unwrap()).await {
        Ok(_) =>
            {
                let payload = PayloadNoData {
                    message: "Delete Job success".to_string()
                };
                Ok(warp::reply::with_status(warp::reply::json(&payload), StatusCode::OK))
            }
        Err(e) => Err(warp::reject::custom(e)),
    }
}
