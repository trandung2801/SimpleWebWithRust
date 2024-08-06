use serde_json::json;
use crate::middleware::jwt::Claims;
use crate::models::resume::{ResumeInfo, ResumeMac};
use crate::models::store::Store;

pub async fn create_resume(store: Store, claims: Claims, resume_info: ResumeInfo)
                            -> Result<impl warp::Reply, warp::Rejection>
{
    // let new_resume_user_id = resume_info.
    let new_resume = ResumeInfo {
        user_id: resume_info.user_id,
        email: resume_info.email,
        url: resume_info.url,
        status: resume_info.status,
    };
    match ResumeMac::create(store, new_resume) {
        Ok(res) =>
            {
                let payload = json!({
                    "statusCode": 201,
                    "message": "Create company success",
                    "data": res
                });
                Ok(warp::reply::json(&payload))
            }
        Err(e) => Err(warp::reject::custom(e)),
    }
}