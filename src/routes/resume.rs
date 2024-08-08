use warp::Filter;
use crate::controllers::resume::{create_resume, delete_resume, get_list_resume_by_job, get_list_resume_by_user_id, get_resume, update_resume};
use crate::middleware::authen::auth;
use crate::models::role::{ADMIN_ROLE_ID, HR_ROLE_ID, USER_ROLE_ID};
use crate::models::store::Store;

pub fn resume_route(base_path: &'static str, store: Store)
                     -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    let resume_path = warp::path(base_path)
                        .and(warp::path("v1"))
                        .and(warp::path("resume"));

    let store_filter = warp::any().map(move || store.clone());

    ///POST api/v1/resume/createResume
    let create_api = resume_path
        .and(warp::path("createResume"))
        .and(warp::path::end())
        .and(warp::post())
        .and(store_filter.clone())
        .and(auth(USER_ROLE_ID))
        .and(warp::body::json())
        .and_then(create_resume);

    ///GET api/v1/resume/getResume/:id
    let get_resume_api = resume_path
        .and(warp::get())
        .and(warp::path("getResume"))
        .and(store_filter.clone())
        .and(auth(USER_ROLE_ID))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and_then(get_resume);

    ///GET api/v1/resume/listResume?limit=x&offset=y
    let get_list_resume_user_api = resume_path
        .and(warp::get())
        .and(warp::path("listResume"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(auth(USER_ROLE_ID))
        .and(warp::query())
        .and_then(get_list_resume_by_user_id);

    ///GET api/v1/resume/listResume?limit=x&offset=y
    let get_list_resume_job_api = resume_path
        .and(warp::get())
        .and(warp::path("listResume"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::query())
        .and_then(get_list_resume_by_job);

    ///PUT api/v1/resume/updateResume
    let update_resume_api = resume_path
        .and(warp::path("updateResume"))
        .and(warp::path::end())
        .and(warp::put())
        .and(store_filter.clone())
        .and(auth(USER_ROLE_ID))
        .and(warp::body::json())
        .and_then(update_resume);

    ///PUT api/v1/resume/deleteResume
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