use std::collections::HashMap;
use std::sync::Arc;

use tracing::instrument;
use warp::http::StatusCode;

use crate::errors::Error;
use crate::models::job::JobId;
use crate::models::pagination::{Pagination, PaginationForJob};
use crate::models::resume::{NewResume, Resume, ResumeId};
use crate::models::store_trait::StoreMethods;
use crate::services::jwt::Claims;
use crate::utils::convert_to_json::{Data, PayloadNoData, PayloadWithData};

// Handle for create resume
#[instrument(level = "info", skip(store))]
pub async fn create_resume(
    store: Arc<dyn StoreMethods + Send + Sync>,
    claims: Claims,
    new_resume: NewResume,
) -> Result<impl warp::Reply, warp::Rejection> {
    let res = store.create_resume(new_resume).await.map_err(Error::from)?;
    let payload = PayloadWithData {
        message: "Success".to_string(),
        data: Data::Resume(res),
    };
    Ok(warp::reply::with_status(
        warp::reply::json(&payload),
        StatusCode::CREATED,
    ))
}

// Handle for retrieving resume by ID
#[instrument(level = "info", skip(store))]
pub async fn get_resume(
    store: Arc<dyn StoreMethods + Send + Sync>,
    claims: Claims,
    resume_id: i32,
) -> Result<impl warp::Reply, warp::Rejection> {
    let res = store
        .get_resume_by_id(ResumeId(resume_id))
        .await
        .map_err(Error::from)?;
    let payload = PayloadWithData {
        message: "Success".to_string(),
        data: Data::Resume(res),
    };
    Ok(warp::reply::with_status(
        warp::reply::json(&payload),
        StatusCode::OK,
    ))
}

// Handle for retrieving list resumes based on query parameters by user ID
#[instrument(level = "info", skip(store))]
pub async fn get_list_resume_by_user_id(
    store: Arc<dyn StoreMethods + Send + Sync>,
    claims: Claims,
    params: HashMap<String, String>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut pagination = Pagination::default();

    if !params.is_empty() {
        pagination = Pagination::extract_pagination(params)?;
    }
    let res = store
        .get_list_resume_by_user_id(pagination.limit, pagination.offset, claims.id)
        .await
        .map_err(Error::from)?;
    let payload = PayloadWithData {
        message: "Success".to_string(),
        data: Data::ListResume(res),
    };
    Ok(warp::reply::with_status(
        warp::reply::json(&payload),
        StatusCode::OK,
    ))
}

// Handle for retrieving list resumes based on query parameters by job ID
#[instrument(level = "info", skip(store))]
pub async fn get_list_resume_by_job(
    store: Arc<dyn StoreMethods + Send + Sync>,
    params: HashMap<String, String>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut pagination = PaginationForJob::default();

    if !params.is_empty() {
        // event!(Level::INFO, pagination = true);
        pagination = PaginationForJob::extract_pagination_job(params)?;
    }
    let res = store
        .get_list_resume_by_job_id(
            pagination.limit,
            pagination.offset,
            JobId(pagination.job_id),
        )
        .await
        .map_err(Error::from)?;
    let mut resume_list = Vec::new();
    for element in res {
        let resume = store.clone().get_resume_by_id(element.resume_id).await?;
        resume_list.push(resume);
    }
    let payload = PayloadWithData {
        message: "Success".to_string(),
        data: Data::ListResume(resume_list),
    };
    Ok(warp::reply::with_status(
        warp::reply::json(&payload),
        StatusCode::OK,
    ))
}

// Handler for updating resume.
#[instrument(level = "info", skip(store))]
pub async fn update_resume(
    store: Arc<dyn StoreMethods + Send + Sync>,
    claims: Claims,
    resume: Resume,
) -> Result<impl warp::Reply, warp::Rejection> {
    // Check valid of resume update
    if claims.id != resume.user_id.clone() {
        let payload = PayloadNoData {
            message: "Can't update".to_string(),
        };
        return Ok(warp::reply::with_status(
            warp::reply::json(&payload),
            StatusCode::BAD_REQUEST,
        ));
    }
    let res = store.update_resume(resume).await.map_err(Error::from)?;
    let payload = PayloadWithData {
        message: "Success".to_string(),
        data: Data::Resume(res),
    };
    Ok(warp::reply::with_status(
        warp::reply::json(&payload),
        StatusCode::OK,
    ))
}

// Handler for deleting resume by ID.
#[instrument(level = "info", skip(store))]
pub async fn delete_resume(
    store: Arc<dyn StoreMethods + Send + Sync>,
    claims: Claims,
    resume: Resume,
) -> Result<impl warp::Reply, warp::Rejection> {
    let _ = store
        .delete_resume(resume.id.unwrap())
        .await
        .map_err(Error::from)?;
    Ok(warp::reply::with_status(
        "Success".to_string(),
        StatusCode::OK,
    ))
}
