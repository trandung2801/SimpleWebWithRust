use warp::{Filter};
use crate::controllers::userController::{get_user, get_list_users,
                                         register, login,
                                         update_user, delete,
                                         update_password, set_admin_role};
use crate::middleware::authen::auth;
use crate::models::store::Store;


pub fn user_route(base_path: &'static str, store: Store)
                 -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let user_path = warp::path(base_path).and(warp::path("v1"));

    // let db_filter = warp::any().map(move || db.clone());
    let store_filter = warp::any().map(move || store.clone());

    let register_api = user_path
        .and(warp::path("register"))
        .and(warp::post())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(register);

    let login_api = user_path
        .and(warp::path("login"))
        .and(warp::post())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(login);

    let get_list_user_api = user_path
        .and(warp::get())
        .and(warp::path("user"))
        .and(warp::path("listUser"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(get_list_users);

    let get_user_api = user_path
        .and(warp::get())
        .and(warp::path("user"))
        .and(warp::path("getUser"))
        .and(store_filter.clone())
        .and(warp::path::param())
        .and_then(get_user);

    let update_user_api = user_path
        .and(warp::path("user"))
        .and(warp::path("updateUser"))
        .and(warp::put())
        .and(store_filter.clone())
        .and(auth(false))
        .and(warp::body::json())
        .and_then(update_user);

    let update_user_password_api = user_path
        .and(warp::path("user"))
        .and(warp::path("updatePassword"))
        .and(warp::put())
        .and(store_filter.clone())
        .and(auth(false))
        .and(warp::body::json())
        .and_then(update_password);

    let update_admin_password_api = user_path
        .and(warp::path("admin"))
        .and(warp::path("updatePassword"))
        .and(warp::put())
        .and(store_filter.clone())
        .and(auth(true))
        .and(warp::body::json())
        .and_then(update_password);

    let set_admin_api = user_path
        .and(warp::path("admin"))
        .and(warp::path("setAdmin"))
        .and(warp::put())
        .and(store_filter.clone())
        .and(auth(true))
        .and(warp::body::json())
        .and_then(set_admin_role);

    let delete_user_api = user_path
        .and(warp::path("user"))
        .and(warp::path("deleteUser"))
        .and(warp::delete())
        .and(store_filter.clone())
        .and(auth(false))
        .and(warp::body::json())
        .and_then(delete);

    get_list_user_api
        .or(get_user_api)
        .or(register_api)
        .or(login_api)
        .or(update_user_api)
        .or(update_user_password_api)
        .or(update_admin_password_api)
        .or(set_admin_api)
        .or(delete_user_api)
}
