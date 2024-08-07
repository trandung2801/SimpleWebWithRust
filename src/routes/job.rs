use crate::models::store::Store;
use warp::{Filter};
use crate::controllers::job::create_job;
use crate::middleware::authen::auth;
use crate::models::role::{HR_ROLE_ID, USER_ROLE_ID};

pub fn job_route(base_path: &'static str, store: Store)
                 -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    let job_path = warp::path(base_path)
        .and(warp::path("v1"))
        .and(warp::path("job"));
    let store_filter = warp::any().map(move || store.clone());

    ///POST api/v1/resume/createJob
    let create_api = job_path
        .and(warp::path("createJob"))
        .and(warp::path::end())
        .and(warp::post())
        .and(store_filter.clone())
        .and(auth(HR_ROLE_ID))
        .and(warp::body::json())
        .and_then(create_job);
    //
    // ///GET api/v1/resume/getJob/:id
    // let get_resume_api = job_path
    //     .and(warp::get())
    //     .and(warp::path("getJob"))
    //     .and(store_filter.clone())
    //     .and(auth(HR_ROLE_ID))
    //     .and(warp::path::param::<i32>())
    //     .and(warp::path::end())
    //     .and_then();
    //
    // ///GET api/v1/resume/listJob?limit=x&offset=y
    // let get_list_resume_user_api = job_path
    //     .and(warp::get())
    //     .and(warp::path("listJob"))
    //     .and(warp::path::end())
    //     .and(store_filter.clone())
    //     .and(auth(HR_ROLE_ID))
    //     .and(warp::query())
    //     .and_then();

    // ///GET api/v1/resume/listJob?limit=x&offset=y
    // let get_list_resume_job_api = job_path
    //     .and(warp::get())
    //     .and(warp::path("listJob"))
    //     .and(warp::path::end())
    //     .and(store_filter.clone())
    //     .and(auth(HR_ROLE_ID))
    //     .and(warp::query())
    //     .and_then(get_list_resume_by_job);

    // ///PUT api/v1/resume/updateJob
    // let update_resume_api = job_path
    //     .and(warp::path("updateJob"))
    //     .and(warp::path::end())
    //     .and(warp::put())
    //     .and(store_filter.clone())
    //     .and(auth(HR_ROLE_ID))
    //     .and(warp::body::json())
    //     .and_then();
    //
    // ///PUT api/v1/resume/deleteJob
    // let delete_resume_api = job_path
    //     .and(warp::path("deleteJob"))
    //     .and(warp::path::end())
    //     .and(warp::put())
    //     .and(store_filter.clone())
    //     .and(auth(HR_ROLE_ID))
    //     .and(warp::body::json())
    //     .and_then();

}