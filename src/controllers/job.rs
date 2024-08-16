use std::collections::HashMap;
use std::sync::Arc;
use reqwest::StatusCode;
use tracing::{event, instrument, Level};
use crate::service::convert_to_json::{Data, PayloadNoData, PayloadWithData};
use crate::service::jwt::Claims;
use crate::models::job::{NewJob, JobId, Job};
use crate::models::map_resume_job::NewMapResumeJob;
use crate::models::store_trait::StoreMethods;
use crate::models::pagination::{Pagination, PaginationMethods};

// Handle for create job
//
// This function adds a new job to the system. It takes job information to
// be created and a reference to the StoreMethods trait object for job. It
// returns a success response with status 200 if job is created successfully
#[instrument(level = "info", skip(store))]
pub async fn create_job(store: Arc<dyn StoreMethods + Send + Sync>, claims: Claims, new_job: NewJob)
                        -> Result<impl warp::Reply, warp::Rejection>
{
    // Check authorization create job of the user
    match store.get_user_by_id(claims.id).await {
        Ok(res) => {
            if res.company_id != new_job.clone().company_id {
                let payload = PayloadNoData {
                    message: "Un authorization create job".to_string(),
                };
                return Ok(warp::reply::with_status(
                    warp::reply::json(&payload),
                    StatusCode::BAD_REQUEST))
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
    match store.create_job(job).await {
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

// Handle for retrieving job by ID
//
// This function retrieves a job with the specified ID from the system. It takes
// the ID from query parameters. It returns a JSON response containing the job.
#[instrument(level = "info", skip(store))]
pub async fn get_job(store: Arc<dyn StoreMethods + Send + Sync>, job_id: i32)
                        -> Result<impl warp::Reply, warp::Rejection>
{
    match store.get_job_by_id(JobId(job_id)).await {
        Ok(res) =>
            {
                let payload = PayloadWithData {
                    message: "Get Job Success".to_string(),
                    data: Data::Job(res)
                };
                Ok(warp::reply::with_status(warp::reply::json(&payload), StatusCode::OK))
            }
        Err(e) => Err(warp::reject::custom(e)),
    }
}

// Handle for retrieving list jobs based on query parameters
//
// This function retrieves list jobs based on the provided query parameters. It takes a HashMap
// containing the query parameters and a reference to the StoreMethod trait object for job.
// It returns a JSON response containing the list of jobs.
#[instrument(level = "info", skip(store))]
pub async fn get_list_job(store: Arc<dyn StoreMethods + Send + Sync>, params: HashMap<String, String>)
                                        -> Result<impl warp::Reply, warp::Rejection>
{
    // Get pagination from query parameters
    let mut pagination = Pagination::default();

    if !params.is_empty() {
        event!(Level::INFO, pagination = true);
        pagination = <Pagination as PaginationMethods>::extract_pagination(params)?;
    }
    // Get list jobs with pagination filters
    match store.get_list_job(pagination.limit, pagination.offset).await {
        Ok(res) =>
            {
                let payload = PayloadWithData {
                    message: "Get List Job Success".to_string(),
                    data: Data::ListJob(res)
                };
                Ok(warp::reply::with_status(warp::reply::json(&payload), StatusCode::OK))
            }
        Err(e) => Err(warp::reject::custom(e)),
    }
}

// Handler for updating job.
//
// This function updates job. It takes info of the job to be updated
// from Job and a reference to the StoreMethod trait object for job.
// It returns a success response with status code 200 if the job update successfully
#[instrument(level = "info", skip(store))]
pub async fn update_job(store: Arc<dyn StoreMethods + Send + Sync>, claims: Claims, job: Job)
                           -> Result<impl warp::Reply, warp::Rejection>
{
    // Check authorization update job of the user
    match store.get_user_by_id(claims.id).await {
        Ok(res) => {
            if res.company_id != job.clone().company_id {
                let payload = PayloadNoData {
                    message: "Un authorization update job".to_string(),
                };
                return Ok(warp::reply::with_status(
                    warp::reply::json(&payload),
                    StatusCode::BAD_REQUEST))
            }
        }
        _ => ()
    }
    match store.update_job(job).await {
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

// Handler for apply job.
//
// This function create new map resume vs job. It takes info of relationship of resume and job
// to be created and a reference to the StoreMethod trait object for MapResumeJob.
// It returns a success response with status code 200 if the map resume vs job successfully
#[instrument(level = "info", skip(store))]
pub async fn apply_job(store: Arc<dyn StoreMethods + Send + Sync>, claims: Claims, new_map_resume_job: NewMapResumeJob)
                        -> Result<impl warp::Reply, warp::Rejection>
{
    // Check authorization apply job of the user
    match store.get_resume_by_id(new_map_resume_job.resume_id.clone()).await {
        Ok(res) => {
            if claims.id != res.user_id {
                let payload = PayloadNoData {
                    message: "Invalid user, can't apply job".to_string(),
                };
                return Ok(warp::reply::with_status(
                    warp::reply::json(&payload),
                    StatusCode::BAD_REQUEST))
            }
        }
        _ => ()
    }
    // Check job status
    match store.get_job_by_id(new_map_resume_job.job_id.clone()).await {
        Ok(res) => {
            if res.is_delete == true {
                let payload = PayloadNoData {
                    message: "Job was deleted, can't apply job".to_string(),
                };
                return Ok(warp::reply::with_status(
                    warp::reply::json(&payload),
                    StatusCode::BAD_REQUEST))
            }
        }
        _ => ()
    }
    match store.create_map_job_resume(new_map_resume_job).await {
        Ok(res) =>
            {
                let payload = PayloadWithData {
                    message: "Apply Job Success".to_string(),
                    data: Data::MapJobResume(res)
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
pub async fn delete_job(store: Arc<dyn StoreMethods + Send + Sync>, claims: Claims, job: Job)
                           -> Result<impl warp::Reply, warp::Rejection>
{
    // Check authorization delete job of the user
    match store.get_user_by_id(claims.id).await {
        Ok(res) => {
            if res.company_id != job.clone().company_id {
                return Ok(warp::reply::with_status(
                    "Un authorization delete job".to_string(),
                    StatusCode::BAD_REQUEST))
            }
        }
        _ => ()
    }
    match store.delete_job(job.id.unwrap()).await {
        Ok(_) =>
            {
                Ok(warp::reply::with_status("Delete Job Success".to_string(), StatusCode::OK))
            }
        Err(e) => Err(warp::reject::custom(e)),
    }
}
