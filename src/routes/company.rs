use crate::controllers::company::{
    create_company, delete_company, get_company, get_list_company, update_company,
};
use crate::middleware::authen::auth;
use crate::models::role::ADMIN_ROLE_ID;
use crate::models::store_trait::StoreMethods;
use std::sync::Arc;
use warp::Filter;

// Configures and returns the Warp filter for handling HTTP requests of company
pub fn company_route(
    base_path: &'static str,
    store: Arc<dyn StoreMethods + Send + Sync>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    //Add base path into path
    let company_path = warp::path(base_path)
        .and(warp::path("v1"))
        .and(warp::path("company"));
    //Configures store filter
    let store_filter = warp::any().map(move || store.clone());

    //POST api/v1/company/createCompany
    let create_api = company_path
        .and(warp::path("createCompany"))
        .and(warp::path::end())
        .and(warp::post())
        .and(store_filter.clone())
        .and(auth(ADMIN_ROLE_ID))
        .and(warp::body::json())
        .and_then(create_company);

    //GET api/v1/company/listCompany?limit=x&offset=y
    let get_list_company_api = company_path
        .and(warp::get())
        .and(warp::path("listCompany"))
        .and(store_filter.clone())
        .and(warp::path::end())
        .and(warp::query())
        .and_then(get_list_company);

    //GET api/v1/company/getCompany/:id
    let get_company_api = company_path
        .and(warp::get())
        .and(warp::path("getCompany"))
        .and(store_filter.clone())
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and_then(get_company);

    //PUT api/v1/company/updateCompany
    let update_company_api = company_path
        .and(warp::path("updateCompany"))
        .and(warp::path::end())
        .and(warp::put())
        .and(store_filter.clone())
        .and(auth(ADMIN_ROLE_ID))
        .and(warp::body::json())
        .and_then(update_company);

    //PUT api/v1/company/deleteCompany
    let delete_company_api = company_path
        .and(warp::path("deleteCompany"))
        .and(warp::path::end())
        .and(warp::put())
        .and(store_filter.clone())
        .and(auth(ADMIN_ROLE_ID))
        .and(warp::body::json())
        .and_then(delete_company);

    get_list_company_api
        .or(get_company_api)
        .or(create_api)
        .or(update_company_api)
        .or(delete_company_api)
}

// #[cfg(test)]
// #[path = "../tests/route_company.rs"]
// mod route_company_tests;
