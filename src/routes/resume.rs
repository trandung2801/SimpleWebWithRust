use std::sync::Arc;
use warp::Filter;
use crate::controllers::resume::{create_resume, delete_resume, get_list_resume_by_job, get_list_resume_by_user_id, get_resume, update_resume};
use crate::middleware::authen::auth;
use crate::models::role::{USER_ROLE_ID};
use crate::models::store_trait::StoreMethods;

// Configures and returns the Warp filter for handling HTTP requests of job
pub fn resume_route(base_path: &'static str, store: Arc<dyn StoreMethods + Send + Sync>)
                     -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    //Add base path into path
    let resume_path = warp::path(base_path)
                        .and(warp::path("v1"))
                        .and(warp::path("resume"));
    //Configures store filter
    let store_filter = warp::any().map(move || store.clone());

    //POST api/v1/resume/createResume
    let create_api = resume_path
        .and(warp::path("createResume"))
        .and(warp::path::end())
        .and(warp::post())
        .and(store_filter.clone())
        .and(auth(USER_ROLE_ID))
        .and(warp::body::json())
        .and_then(create_resume);

    //GET api/v1/resume/getResume/:id
    let get_resume_api = resume_path
        .and(warp::get())
        .and(warp::path("getResume"))
        .and(store_filter.clone())
        .and(auth(USER_ROLE_ID))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and_then(get_resume);

    //GET api/v1/resume/listResumeByUser?limit=x&offset=y
    let get_list_resume_user_api = resume_path
        .and(warp::get())
        .and(warp::path("listResumeByUser"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(auth(USER_ROLE_ID))
        .and(warp::query())
        .and_then(get_list_resume_by_user_id);

    //GET api/v1/resume/listResumeByJob?limit=x&offset=y&jobId=z
    let get_list_resume_job_api = resume_path
        .and(warp::get())
        .and(warp::path("listResumeByJob"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::query())
        .and_then(get_list_resume_by_job);

    //PUT api/v1/resume/updateResume
    let update_resume_api = resume_path
        .and(warp::path("updateResume"))
        .and(warp::path::end())
        .and(warp::put())
        .and(store_filter.clone())
        .and(auth(USER_ROLE_ID))
        .and(warp::body::json())
        .and_then(update_resume);

    //PUT api/v1/resume/deleteResume
    let delete_resume_api = resume_path
        .and(warp::path("deleteResume"))
        .and(warp::path::end())
        .and(warp::put())
        .and(store_filter.clone())
        .and(auth(USER_ROLE_ID))
        .and(warp::body::json())
        .and_then(delete_resume);



    create_api
        .or(get_resume_api)
        .or(get_list_resume_user_api)
        .or(get_list_resume_job_api)
        .or(update_resume_api)
        .or(delete_resume_api)
}
// #[cfg(test)]
// #[path = "../tests/route_resume.rs"]
// mod route_resume_tests;