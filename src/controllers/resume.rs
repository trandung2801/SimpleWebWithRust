use std::collections::HashMap;
use std::sync::Arc;
use tracing::{event, instrument, Level};
use warp::http::StatusCode;
use crate::middleware::convert_to_json::{Data, PayloadNoData, PayloadWithData};
use crate::middleware::jwt::Claims;
use crate::models::job::JobId;
use crate::models::pagination::{Pagination, PaginationForJob, PaginationMethods};
use crate::models::resume::{NewResume, ResumeId, Resume};
use crate::models::store_trait::StoreMethods;

#[instrument(level = "info")]
pub async fn create_resume(store: Arc<dyn StoreMethods + Send + Sync>, claims: Claims, new_resume: NewResume)
                           -> Result<impl warp::Reply, warp::Rejection>
{
    let resume = NewResume {
        user_id: new_resume.user_id,
        email: new_resume.email,
        url: new_resume.url
    };
    match store.create_resume(resume).await {
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

#[instrument(level = "info")]
pub async fn get_resume(store: Arc<dyn StoreMethods + Send + Sync>, claims: Claims, resume_id: i32)
    -> Result<impl warp::Reply, warp::Rejection>
{
    match store.get_resume_by_id(ResumeId(resume_id)).await {
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

#[instrument(level = "info")]
pub async fn get_list_resume_by_user_id(store: Arc<dyn StoreMethods + Send + Sync>, claims: Claims, params: HashMap<String, String>)
    -> Result<impl warp::Reply, warp::Rejection>
{
    let mut pagination = Pagination::default();

    if !params.is_empty() {
        event!(Level::INFO, pagination = true);
        pagination = <Pagination as PaginationMethods>::extract_pagination(params)?;
    }
    match store.get_list_resume_by_user_id(pagination.limit, pagination.offset, claims.id).await {
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

#[instrument(level = "info")]
pub async fn get_list_resume_by_job(store: Arc<dyn StoreMethods + Send + Sync>, params: HashMap<String, String>)
                                        -> Result<impl warp::Reply, warp::Rejection>
{

    let mut pagination = PaginationForJob::default();

    if !params.is_empty() {
        event!(Level::INFO, pagination = true);
        pagination = <Pagination as PaginationMethods>::extract_pagination_job(params)?;
    }

    match store.get_list_resume_by_job_id(pagination.limit, pagination.offset, JobId(pagination.job_id)).await {
        Ok(res) =>
            {
                let mut resume_list= Vec::new();
                for e in res {
                    let resume =  store.clone().get_resume_by_id(e.resume_id).await?;
                    resume_list.push(resume);
                }
                let payload = PayloadWithData {
                    message: "Get List Resume Success".to_string(),
                    data: Data::ListResume(resume_list)
                };
                Ok(warp::reply::with_status(warp::reply::json(&payload), StatusCode::OK))
            }
        Err(e) => Err(warp::reject()),
    }
}

#[instrument(level = "info")]
pub async fn update_resume(store: Arc<dyn StoreMethods + Send + Sync>, claims: Claims, resume: Resume)
    -> Result<impl warp::Reply, warp::Rejection>
{
    if claims.id != resume.clone().user_id {
        let payload = PayloadNoData {
            message: "Can't update".to_string(),
        };
        return Ok(warp::reply::with_status(warp::reply::json(&payload), StatusCode::BAD_REQUEST))
    }
    match store.update_resume(resume).await {
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

#[instrument(level = "info")]
pub async fn delete_resume(store: Arc<dyn StoreMethods + Send + Sync>, claims: Claims, resume: Resume)
                            -> Result<impl warp::Reply, warp::Rejection>
{
    match store.delete_resume(resume.id.unwrap()).await {
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
