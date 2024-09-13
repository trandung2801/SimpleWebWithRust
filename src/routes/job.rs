use std::sync::Arc;

use warp::Filter;

use crate::controllers::job::{
    apply_job, create_job, delete_job, get_job, get_list_job, update_job,
};
use crate::middleware::authen::auth;
use crate::models::role::{HR_ROLE_ID, USER_ROLE_ID};
use crate::models::store_trait::StoreMethods;

// Configures and returns the Warp filter for handling HTTP requests of job
pub fn job_route(
    base_path: &'static str,
    store: Arc<dyn StoreMethods + Send + Sync>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    //Add base path into path
    let job_path = warp::path(base_path)
        .and(warp::path("v1"))
        .and(warp::path("job"));
    //Configures store filter
    let store_filter = warp::any().map(move || store.clone());

    //POST api/v1/job/createJob
    let create_api = job_path
        .and(warp::path("create-job"))
        .and(warp::path::end())
        .and(warp::post())
        .and(store_filter.clone())
        .and(auth(HR_ROLE_ID))
        .and(warp::body::json())
        .and_then(create_job);

    //GET api/v1/job/getJob/:id
    let get_job_api = job_path
        .and(warp::get())
        .and(warp::path("get-job"))
        .and(store_filter.clone())
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and_then(get_job);

    //GET api/v1/job/listJob?limit=x&offset=y
    let get_list_job_api = job_path
        .and(warp::get())
        .and(warp::path("list-job"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::query())
        .and_then(get_list_job);

    //PUT api/v1/job/updateJob
    let update_job_api = job_path
        .and(warp::path("update-job"))
        .and(warp::path::end())
        .and(warp::put())
        .and(store_filter.clone())
        .and(auth(HR_ROLE_ID))
        .and(warp::body::json())
        .and_then(update_job);

    //POST api/v1/job/applyJob
    let apply_job_api = job_path
        .and(warp::path("apply-job"))
        .and(warp::path::end())
        .and(warp::post())
        .and(store_filter.clone())
        .and(auth(USER_ROLE_ID))
        .and(warp::body::json())
        .and_then(apply_job);

    //PUT api/v1/job/deleteJob
    let delete_job_api = job_path
        .and(warp::path("delete-job"))
        .and(warp::path::end())
        .and(warp::put())
        .and(store_filter.clone())
        .and(auth(HR_ROLE_ID))
        .and(warp::body::json())
        .and_then(delete_job);

    create_api
        .or(get_job_api)
        .or(get_list_job_api)
        .or(update_job_api)
        .or(apply_job_api)
        .or(delete_job_api)
}
