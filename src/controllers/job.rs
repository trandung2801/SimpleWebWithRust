use reqwest::StatusCode;
use crate::middleware::convert_to_json::{Data, PayloadNoData, PayloadWithData};
use crate::middleware::jwt::Claims;
use crate::models::job::{JobMac, NewJob, JobActions};
use crate::models::store::Store;
use crate::models::user::{UserMac, UserActions};

pub async fn create_job(store: Store, claims: Claims, new_job: NewJob)
                        -> Result<impl warp::Reply, warp::Rejection>
{
    match UserMac::get_by_id(store.clone(), claims.id).await {
        Ok(res) => {
            if res.company_id != new_job.clone().company_id {
                let payload = PayloadNoData {
                    status_code: StatusCode::BAD_REQUEST,
                    message: "Can't update".to_string(),
                };
                return Ok(warp::reply::json(&payload))
            }
        }
        _ => ()
    }

    let job = NewJob {
        name: new_job.name,
        company_id: new_job.company_id,
        location: new_job.location,
        quantity: new_job.quantity,
        salary: new_job.salary,
        level: new_job.level,
        description: new_job.description
    };
    match JobMac::create(store, job) {
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