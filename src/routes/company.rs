use warp::{Filter};
use crate::middleware::authen::auth;
use crate::models::store::Store;
use crate::controllers::company;
use crate::controllers::company::{create_company, delete_company, get_company, get_list_company, update_company};
use crate::controllers::user::register;

pub fn company_route(base_path: &'static str, store: Store)
                     -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    let company_path = warp::path(base_path)
                                                    .and(warp::path("v1"))
                                                    .and(warp::path("company"));
    
    let store_filter = warp::any().map(move || store.clone());

    let create_api = company_path
        .and(warp::path("createCompany"))
        .and(warp::post())
        .and(store_filter.clone())
        .and(auth(1))
        .and(warp::body::json())
        .and_then(create_company);

    let get_list_company_api = company_path
        .and(warp::get())
        .and(warp::path("listCompany"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(get_list_company);

    let get_company_api = company_path
        .and(warp::get())
        .and(warp::path("getCompany"))
        .and(store_filter.clone())
        .and(warp::path::param())
        .and_then(get_company);

    let update_company_api = company_path
        .and(warp::path("updateCompany"))
        .and(warp::put())
        .and(store_filter.clone())
        .and(auth(1))
        .and(warp::body::json())
        .and_then(update_company);

    let delete_company_api = company_path
        .and(warp::path("deleteCompany"))
        .and(warp::delete())
        .and(store_filter.clone())
        .and(auth(1))
        .and(warp::body::json())
        .and_then(delete_company);

    get_list_company_api
        .or(get_company_api)
        .or(create_api)
        .or(update_company_api)
        .or(delete_company_api)
}
