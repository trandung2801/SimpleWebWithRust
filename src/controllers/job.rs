use std::collections::HashMap;
use std::sync::Arc;

use reqwest::StatusCode;
use tracing::instrument;

use crate::errors::Error;
use crate::models::job::{Job, JobId, NewJob};
use crate::models::map_resume_job::NewMapResumeJob;
use crate::models::pagination::Pagination;
use crate::models::store_trait::StoreMethods;
use crate::services::jwt::Claims;
use crate::utils::convert_to_json::{Data, PayloadNoData, PayloadWithData};

// Handle for create job
#[instrument(level = "info", skip(store))]
pub async fn create_job(
    store: Arc<dyn StoreMethods + Send + Sync>,
    claims: Claims,
    new_job: NewJob,
) -> Result<impl warp::Reply, warp::Rejection> {
    // Check authorization create job of the user
    let user = store.get_user_by_id(claims.id).await?;
    if user.company_id != new_job.company_id.clone() {
        let payload = PayloadNoData {
            message: "Un authorization create job".to_string(),
        };
        return Ok(warp::reply::with_status(
            warp::reply::json(&payload),
            StatusCode::BAD_REQUEST,
        ));
    }
    let res = store.create_job(new_job).await.map_err(Error::from)?;
    let payload = PayloadWithData {
        message: "Success".to_string(),
        data: Data::Job(res),
    };
    Ok(warp::reply::with_status(
        warp::reply::json(&payload),
        StatusCode::CREATED,
    ))
}

// Handle for retrieving job by ID
#[instrument(level = "info", skip(store))]
pub async fn get_job(
    store: Arc<dyn StoreMethods + Send + Sync>,
    job_id: i32,
) -> Result<impl warp::Reply, warp::Rejection> {
    let res = store
        .get_job_by_id(JobId(job_id))
        .await
        .map_err(Error::from)?;
    let payload = PayloadWithData {
        message: "Success".to_string(),
        data: Data::Job(res),
    };
    Ok(warp::reply::with_status(
        warp::reply::json(&payload),
        StatusCode::OK,
    ))
}

// Handle for retrieving list jobs based on query parameters
#[instrument(level = "info", skip(store))]
pub async fn get_list_job(
    store: Arc<dyn StoreMethods + Send + Sync>,
    params: HashMap<String, String>,
) -> Result<impl warp::Reply, warp::Rejection> {
    // Get pagination from query parameters
    let mut pagination = Pagination::default();

    if !params.is_empty() {
        pagination = Pagination::extract_pagination(params)?;
    }
    // Get list jobs with pagination filters
    let res = store
        .get_list_job(pagination.limit, pagination.offset)
        .await
        .map_err(Error::from)?;
    let payload = PayloadWithData {
        message: "Success".to_string(),
        data: Data::ListJob(res),
    };
    Ok(warp::reply::with_status(
        warp::reply::json(&payload),
        StatusCode::OK,
    ))
}

// Handler for updating job.
#[instrument(level = "info", skip(store))]
pub async fn update_job(
    store: Arc<dyn StoreMethods + Send + Sync>,
    claims: Claims,
    job: Job,
) -> Result<impl warp::Reply, warp::Rejection> {
    // Check authorization update job of the user
    if let Ok(res) = store.get_user_by_id(claims.id).await {
        if res.company_id != job.company_id.clone() {
            let payload = PayloadNoData {
                message: "Un authorization update job".to_string(),
            };
            return Ok(warp::reply::with_status(
                warp::reply::json(&payload),
                StatusCode::BAD_REQUEST,
            ));
        }
    }
    let res = store.update_job(job).await.map_err(Error::from)?;
    let payload = PayloadWithData {
        message: "Success".to_string(),
        data: Data::Job(res),
    };
    Ok(warp::reply::with_status(
        warp::reply::json(&payload),
        StatusCode::OK,
    ))
}

// Handler for apply job.
#[instrument(level = "info", skip(store))]
pub async fn apply_job(
    store: Arc<dyn StoreMethods + Send + Sync>,
    claims: Claims,
    new_map_resume_job: NewMapResumeJob,
) -> Result<impl warp::Reply, warp::Rejection> {
    // Check job status
    if let Ok(res) = store.get_job_by_id(new_map_resume_job.job_id.clone()).await {
        if res.is_delete {
            let payload = PayloadNoData {
                message: "Job was deleted, can't apply job".to_string(),
            };
            return Ok(warp::reply::with_status(
                warp::reply::json(&payload),
                StatusCode::BAD_REQUEST,
            ));
        }
    }
    let res = store
        .create_map_job_resume(new_map_resume_job)
        .await
        .map_err(Error::from)?;
    let payload = PayloadWithData {
        message: "Success".to_string(),
        data: Data::MapJobResume(res),
    };
    Ok(warp::reply::with_status(
        warp::reply::json(&payload),
        StatusCode::OK,
    ))
}

// Handler for deleting company by ID.
#[instrument(level = "info", skip(store))]
pub async fn delete_job(
    store: Arc<dyn StoreMethods + Send + Sync>,
    claims: Claims,
    job: Job,
) -> Result<impl warp::Reply, warp::Rejection> {
    if let Ok(res) = store.get_user_by_id(claims.id).await {
        if res.company_id != job.company_id.clone() {
            return Ok(warp::reply::with_status(
                "Un authorization delete job".to_string(),
                StatusCode::BAD_REQUEST,
            ));
        }
    }
    let _ = store
        .delete_job(job.id.unwrap())
        .await
        .map_err(Error::from)?;
    Ok(warp::reply::with_status(
        "Success".to_string(),
        StatusCode::OK,
    ))
}
