use crate::models::store::Store;
use warp::{Filter};
// use crate::controllers::company::{create_company, delete_company, get_company, get_list_company, update_company};
use crate::middleware::authen::auth;

pub fn job_route(base_path: &'static str, store: Store)
                 -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    let job_path = warp::path(base_path)
        .and(warp::path("v1"))
        .and(warp::path("job"));
    let store_filter = warp::any().map(move || store.clone());

    let create_api = job_path
        .and(warp::path("createJob"))
        .and(warp::post())
        .and(store_filter.clone())
        .and(auth(3))
        .and(warp::body::json())
        .and_then();

    let get_list_job_api = job_path
        .and(warp::get())
        .and(warp::path("listJob"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then();

    let get_job_api = job_path
        .and(warp::get())
        .and(warp::path("getJob"))
        .and(store_filter.clone())
        .and(warp::path::param())
        .and_then();

    let update_job_api = job_path
        .and(warp::path("updateJob"))
        .and(warp::put())
        .and(store_filter.clone())
        .and(auth(3))
        .and(warp::body::json())
        .and_then();

    let delete_job_api = job_path
        .and(warp::path("deleteJob"))
        .and(warp::delete())
        .and(store_filter.clone())
        .and(auth(3))
        .and(warp::body::json())
        .and_then();

    create_api
        .or(get_list_job_api)
        .or(get_job_api)
        .or(update_job_api)
        .or(delete_job_api)
}