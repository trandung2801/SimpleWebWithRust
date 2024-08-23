use crate::models::job::JobId;
use crate::models::pagination::{Pagination, PaginationForJob, PaginationMethods};
use crate::models::resume::{NewResume, Resume, ResumeId};
use crate::models::store_trait::StoreMethods;
use crate::service::convert_to_json::{Data, PayloadNoData, PayloadWithData};
use crate::service::jwt::Claims;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{event, instrument, Level};
use warp::http::StatusCode;

// Handle for create resume
//
// This function adds a new resume to the system. It takes resume information to
// be created and a reference to the StoreMethods trait object for resume. It
// returns a success response with status 200 if resume is created successfully
#[instrument(level = "info", skip(store))]
pub async fn create_resume(
    store: Arc<dyn StoreMethods + Send + Sync>,
    claims: Claims,
    new_resume: NewResume,
) -> Result<impl warp::Reply, warp::Rejection> {
    let resume = NewResume {
        user_id: new_resume.user_id,
        email: new_resume.email,
        url: new_resume.url,
    };
    match store.create_resume(resume).await {
        Ok(res) => {
            let payload = PayloadWithData {
                message: "Created Resume Success".to_string(),
                data: Data::Resume(res),
            };
            Ok(warp::reply::with_status(
                warp::reply::json(&payload),
                StatusCode::CREATED,
            ))
        }
        Err(e) => Err(warp::reject::custom(e)),
    }
}

// Handle for retrieving resume by ID
//
// This function retrieves a resume with the specified ID from the system. It takes
// the ID from query parameters. It returns a JSON response containing the resume.
#[instrument(level = "info", skip(store))]
pub async fn get_resume(
    store: Arc<dyn StoreMethods + Send + Sync>,
    claims: Claims,
    resume_id: i32,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.get_resume_by_id(ResumeId(resume_id)).await {
        Ok(res) => {
            let payload = PayloadWithData {
                message: "Get Resume Success".to_string(),
                data: Data::Resume(res),
            };
            Ok(warp::reply::with_status(
                warp::reply::json(&payload),
                StatusCode::OK,
            ))
        }
        Err(e) => Err(warp::reject::custom(e)),
    }
}

// Handle for retrieving list resumes based on query parameters by user ID
//
// This function retrieves list resumes based on the provided query parameters by user ID.
// It takes a HashMap containing the query parameters and a reference to the StoreMethod
// trait object for resume. It returns a JSON response containing the list of resumes.
#[instrument(level = "info", skip(store))]
pub async fn get_list_resume_by_user_id(
    store: Arc<dyn StoreMethods + Send + Sync>,
    claims: Claims,
    params: HashMap<String, String>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut pagination = Pagination::default();

    if !params.is_empty() {
        event!(Level::INFO, pagination = true);
        pagination = <Pagination as PaginationMethods>::extract_pagination(params)?;
    }
    match store
        .get_list_resume_by_user_id(pagination.limit, pagination.offset, claims.id)
        .await
    {
        Ok(res) => {
            let payload = PayloadWithData {
                message: "Get List Resume Success".to_string(),
                data: Data::ListResume(res),
            };
            Ok(warp::reply::with_status(
                warp::reply::json(&payload),
                StatusCode::OK,
            ))
        }
        Err(_e) => Err(warp::reject()),
    }
}

// Handle for retrieving list resumes based on query parameters by job ID
//
// This function retrieves list resumes based on the provided query parameters by job ID.
// It takes a HashMap containing the query parameters and a reference to the StoreMethod
// trait object for resume. It returns a JSON response containing the list of resumes.
#[instrument(level = "info", skip(store))]
pub async fn get_list_resume_by_job(
    store: Arc<dyn StoreMethods + Send + Sync>,
    params: HashMap<String, String>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut pagination = PaginationForJob::default();

    if !params.is_empty() {
        // event!(Level::INFO, pagination = true);
        pagination = <Pagination as PaginationMethods>::extract_pagination_job(params)?;
    }

    match store
        .get_list_resume_by_job_id(
            pagination.limit,
            pagination.offset,
            JobId(pagination.job_id),
        )
        .await
    {
        Ok(res) => {
            let mut resume_list = Vec::new();
            for e in res {
                let resume = store.clone().get_resume_by_id(e.resume_id).await?;
                resume_list.push(resume);
            }
            let payload = PayloadWithData {
                message: "Get List Resume Success".to_string(),
                data: Data::ListResume(resume_list),
            };
            Ok(warp::reply::with_status(
                warp::reply::json(&payload),
                StatusCode::OK,
            ))
        }
        Err(_e) => Err(warp::reject()),
    }
}

// Handler for updating resume.
//
// This function updates resume. It takes info of the resume to be updated
// from Job and a reference to the StoreMethod trait object for resume.
// It returns a success response with status code 200 if the resume update successfully
#[instrument(level = "info", skip(store))]
pub async fn update_resume(
    store: Arc<dyn StoreMethods + Send + Sync>,
    claims: Claims,
    resume: Resume,
) -> Result<impl warp::Reply, warp::Rejection> {
    // Check valid of resume update
    if claims.id != resume.clone().user_id {
        let payload = PayloadNoData {
            message: "Can't update".to_string(),
        };
        return Ok(warp::reply::with_status(
            warp::reply::json(&payload),
            StatusCode::BAD_REQUEST,
        ));
    }
    match store.update_resume(resume).await {
        Ok(res) => {
            let payload = PayloadWithData {
                message: "Update Company Success".to_string(),
                data: Data::Resume(res),
            };
            Ok(warp::reply::with_status(
                warp::reply::json(&payload),
                StatusCode::OK,
            ))
        }
        Err(e) => Err(warp::reject::custom(e)),
    }
}

// Handler for deleting resume by ID.
//
// This function deletes resume with the specified ID from the system. It takes the ID of
// the resume to be deleted from Company and a reference to the StoreMethod trait object. It
// returns a success response with status code 200 if the resume is deleted successfully.
#[instrument(level = "info", skip(store))]
pub async fn delete_resume(
    store: Arc<dyn StoreMethods + Send + Sync>,
    claims: Claims,
    resume: Resume,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.delete_resume(resume.id.unwrap()).await {
        Ok(_) => Ok(warp::reply::with_status(
            "Delete Resume Success".to_string(),
            StatusCode::OK,
        )),
        Err(e) => Err(warp::reject::custom(e)),
    }
}
