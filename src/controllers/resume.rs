use std::collections::HashMap;
use tracing::{event, Level};
use warp::http::StatusCode;
use crate::middleware::convert_to_json::{Data, PayloadNoData, PayloadWithData};
use crate::middleware::jwt::Claims;
use crate::models::job::JobId;
use crate::models::pagination::{Pagination, PaginationForJob, PaginationMethods};
use crate::models::resume::{NewResume, ResumeMac, ResumeActions, ResumeId, Resume};
use crate::models::store::Store;
use crate::models::user::{UserActions};


pub async fn create_resume(store: Store, claims: Claims, new_resume: NewResume)
                           -> Result<impl warp::Reply, warp::Rejection>
{
    let resume = NewResume {
        user_id: new_resume.user_id,
        email: new_resume.email,
        url: new_resume.url
    };
    match ResumeMac::create(store, resume).await {
        Ok(res) =>
            {
                let payload = PayloadWithData {
                    message: "Created Resume Success".to_string(),
                    data: Data::Resume(res)
                };
                Ok(warp::reply::with_status(warp::reply::json(&payload), StatusCode::CREATED))            }
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn get_resume(store: Store, claims: Claims, resume_id: i32)
    -> Result<impl warp::Reply, warp::Rejection>
{
    match ResumeMac::get_by_id(store, ResumeId(resume_id)).await {
        Ok(res) =>
            {
                let payload = PayloadWithData {
                    message: "Get Resume Success".to_string(),
                    data: Data::Resume(res)
                };
                Ok(warp::reply::with_status(warp::reply::json(&payload), StatusCode::OK))
            }
        Err(e) => Err(warp::reject()),
    }
}

pub async fn get_list_resume_by_user_id(store: Store, claims: Claims, params: HashMap<String, String>)
    -> Result<impl warp::Reply, warp::Rejection>
{
    let mut pagination = Pagination::default();

    if !params.is_empty() {
        event!(Level::INFO, pagination = true);
        pagination = <Pagination as PaginationMethods>::extract_pagination(params)?;
    }
    match ResumeMac::list_by_user_id(store, pagination.limit, pagination.offset, claims.id).await {
        Ok(res) =>
            {
                let payload = PayloadWithData {
                    message: "Get List Resume Success".to_string(),
                    data: Data::ListResume(res)
                };
                Ok(warp::reply::with_status(warp::reply::json(&payload), StatusCode::OK))
    }
        Err(e) => Err(warp::reject()),
    }
}

pub async fn get_list_resume_by_job(store: Store, params: HashMap<String, String>)
                                        -> Result<impl warp::Reply, warp::Rejection>
{

    let mut pagination = PaginationForJob::default();

    if !params.is_empty() {
        event!(Level::INFO, pagination = true);
        pagination = <Pagination as PaginationMethods>::extract_pagination_job(params)?;
    }

    match ResumeMac::list_by_job_id(store, pagination.limit, pagination.offset, JobId(pagination.job_id)).await {
        Ok(res) =>
            {
                let payload = PayloadWithData {
                    message: "Get List Resume Success".to_string(),
                    data: Data::ListResume(res)
                };
                Ok(warp::reply::with_status(warp::reply::json(&payload), StatusCode::OK))
            }
        Err(e) => Err(warp::reject()),
    }
}

pub async fn update_resume(store: Store, claims: Claims, resume: Resume)
    -> Result<impl warp::Reply, warp::Rejection>
{
    if claims.id != resume.clone().user_id {
        let payload = PayloadNoData {
            message: "Can't update".to_string(),
        };
        return Ok(warp::reply::with_status(warp::reply::json(&payload), StatusCode::BAD_REQUEST))
    }
    match ResumeMac::update(store, resume).await {
        Ok(res) =>
            {
                let payload = PayloadWithData {
                    message: "Update Company Success".to_string(),
                    data: Data::Resume(res)
                };
                Ok(warp::reply::with_status(warp::reply::json(&payload), StatusCode::OK))
            }
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn delete_resume(store: Store, claims: Claims, resume: Resume)
                            -> Result<impl warp::Reply, warp::Rejection>
{
    match ResumeMac::delete(store, resume.id.unwrap()).await {
        Ok(_) =>
            {
                let payload = PayloadNoData {
                    message: "Delete Resume success".to_string()
                };
                Ok(warp::reply::with_status(warp::reply::json(&payload), StatusCode::OK))
            }
        Err(e) => Err(warp::reject::custom(e)),
    }
}
