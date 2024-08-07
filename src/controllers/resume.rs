use std::collections::HashMap;
use serde_json::json;
use tracing::{event, instrument, Level};
use warp::http::StatusCode;
use crate::middleware::convert_to_json::{Data, PayloadNoData, PayloadWithData};
use crate::middleware::jwt::Claims;
use crate::models::company::{Company, CompanyId, CompanyMac};
use crate::models::pagination::Pagination;
use crate::models::resume::{NewResume, ResumeMac, ResumeActions, ResumeId, Resume};
use crate::models::store::Store;
use crate::models::user::UserId;

pub async fn create_resume(store: Store, claims: Claims, new_resume: NewResume)
                           -> Result<impl warp::Reply, warp::Rejection>
{
    let resume = NewResume {
        user_id: new_resume.user_id,
        email: new_resume.email,
        url: new_resume.url
    };
    match ResumeMac::create(store, resume) {
        Ok(res) =>
            {
                let payload = PayloadWithData {
                    status_code: StatusCode::CREATED,
                    message: "Created Resume Success".to_string(),
                    data: Data::Resume(res)
                };
                Ok(warp::reply::json(&payload))
            }
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn get_resume(store: Store, claims: Claims, resume_id: i32)
    -> Result<impl warp::Reply, warp::Rejection>
{
    match ResumeMac::get_by_id(store, ResumeId(resume_id)).await {
        Ok(res) =>
            {
                // let status_code = StatusCode::OK;
                // let payload = json!({
                //     "statusCode": status_code,
                //     "message": "get company success",
                //      "data": res,
                // });
                let payload = PayloadWithData {
                    status_code: StatusCode::OK,
                    message: "Get Resume Success".to_string(),
                    data: Data::Resume(res)
                };
                Ok(warp::reply::json(&payload))
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
        pagination = crate::models::pagination::PaginationMethods::extract_pagination(params)?;
    }
    match ResumeMac::list_by_user_id(store, pagination.limit, pagination.offset, claims.id).await {
        Ok(res) =>
            {
                // let status_code = StatusCode::OK;
                // let payload = json!({
                //     "statusCode": status_code,
                //     "message": "get list companies success",
                //     "data": res
                // });
                let payload = PayloadWithData {
                    status_code: StatusCode::OK,
                    message: "Get List Resume Success".to_string(),
                    data: Data::ListResume(res)
                };
                Ok(warp::reply::json(&payload))
            }
        Err(e) => Err(warp::reject()),
    }
}

// pub async fn get_list_resume_by_job(store: Store, claims: Claims, params: HashMap<String, String>)
//                                         -> Result<impl warp::Reply, warp::Rejection>
// {
//     let mut pagination = Pagination::default();
//
//     if !params.is_empty() {
//         event!(Level::INFO, pagination = true);
//         pagination = crate::models::pagination::PaginationMethods::extract_pagination(params)?;
//     }
//     match ResumeMac::list_by_job_id(store, pagination.limit, pagination.offset, claims.id).await {
//         Ok(res) =>
//             {
//                 // let status_code = StatusCode::OK;
//                 // let payload = json!({
//                 //     "statusCode": status_code,
//                 //     "message": "get list companies success",
//                 //     "data": res
//                 // });
//                 let payload = PayloadWithData {
//                     status_code: StatusCode::OK,
//                     message: "Get List Resume Success".to_string(),
//                     data: Data::ListResume(res)
//                 };
//                 Ok(warp::reply::json(&payload))
//             }
//         Err(e) => Err(warp::reject()),
//     }
// }

pub async fn update_resume(store: Store, claims: Claims, resume: Resume)
    -> Result<impl warp::Reply, warp::Rejection>
{
    if claims.id != resume.clone().user_id {
        let payload = PayloadNoData {
            status_code: StatusCode::BAD_REQUEST,
            message: "Can't update".to_string(),
        };
        return Ok(warp::reply::json(&payload))
    }
    // let resume_update = Re
    match ResumeMac::update(store, resume).await {
        Ok(res) =>
            {
                // let status_code = StatusCode::OK;
                // let payload = json!({
                //     "statusCode": status_code,
                //     "message": "update company success",
                //     "data": res
                // });
                let payload = PayloadWithData {
                    status_code: StatusCode::OK,
                    message: "Update Company Success".to_string(),
                    data: Data::Resume(res)
                };
                Ok(warp::reply::json(&payload))
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
                // let status_code = StatusCode::OK;
                // let payload = json!({
                //     "statusCode": status_code,
                //     "message": "Delete Resume success",
                // });
                let payload = PayloadNoData {
                    status_code: StatusCode::OK,
                    message: "Delete Resume success".to_string()
                };
                Ok(warp::reply::json(&payload))
            }
        Err(e) => Err(warp::reject::custom(e)),
    }
}
