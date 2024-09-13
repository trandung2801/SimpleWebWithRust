use std::sync::Arc;

use warp::Filter;

use crate::controllers::user::{
    delete, get_list_users, get_user_by_id, login, register, set_admin_role, set_hr_role,
    update_password, update_user,
};
use crate::middleware::authen::auth;
use crate::models::role::{ADMIN_ROLE_ID, HR_ROLE_ID, USER_ROLE_ID};
use crate::models::store_trait::StoreMethods;

// Configures and returns the Warp filter for handling HTTP requests of user.
pub fn user_route(
    base_path: &'static str,
    store: Arc<dyn StoreMethods + Send + Sync>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    //Add base path into path
    let user_path = warp::path(base_path).and(warp::path("v1"));
    //Configures store filter
    let store_filter = warp::any().map(move || store.clone());

    //POST api/v1/register
    let register_api = user_path
        .and(warp::path("register"))
        .and(warp::path::end())
        .and(warp::post())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(register);

    //GET api/v1/login
    let login_api = user_path
        .and(warp::path("login"))
        .and(warp::path::end())
        .and(warp::post())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(login);

    //GET api/v1/user/getUser/:id
    let get_user_api = user_path
        .and(warp::get())
        .and(warp::path("user"))
        .and(warp::path("get-user"))
        .and(store_filter.clone())
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and_then(get_user_by_id);

    //GET api/v1/user/listUser?limit=x&offset=y
    let get_list_user_api = user_path
        .and(warp::get())
        .and(warp::path("user"))
        .and(warp::path("list-user"))
        .and(store_filter.clone())
        .and(warp::path::end())
        .and(warp::query())
        .and_then(get_list_users);

    //PUT api/v1/user/updateUser
    let update_user_api = user_path
        .and(warp::path("user"))
        .and(warp::path("update-user"))
        .and(warp::path::end())
        .and(warp::put())
        .and(store_filter.clone())
        .and(auth(USER_ROLE_ID))
        .and(warp::body::json())
        .and_then(update_user);

    //PUT api/v1/user/updateUser
    let update_hr_api = user_path
        .and(warp::path("user"))
        .and(warp::path("update-user"))
        .and(warp::path::end())
        .and(warp::put())
        .and(store_filter.clone())
        .and(auth(HR_ROLE_ID))
        .and(warp::body::json())
        .and_then(update_user);

    //PUT api/v1/admin/updateAdmin
    let update_admin_api = user_path
        .and(warp::path("admin"))
        .and(warp::path("update-admin"))
        .and(warp::path::end())
        .and(warp::put())
        .and(store_filter.clone())
        .and(auth(ADMIN_ROLE_ID))
        .and(warp::body::json())
        .and_then(update_user);

    //PUT api/v1/user/updatePassword
    let update_user_password_api = user_path
        .and(warp::path("user"))
        .and(warp::path("update-password"))
        .and(warp::path::end())
        .and(warp::put())
        .and(store_filter.clone())
        .and(auth(USER_ROLE_ID))
        .and(warp::body::json())
        .and_then(update_password);

    //PUT api/v1/user/updatePassword
    let update_hr_password_api = user_path
        .and(warp::path("user"))
        .and(warp::path("update-password"))
        .and(warp::path::end())
        .and(warp::put())
        .and(store_filter.clone())
        .and(auth(HR_ROLE_ID))
        .and(warp::body::json())
        .and_then(update_password);

    //PUT api/v1/admin/updatePassword
    let update_admin_password_api = user_path
        .and(warp::path("admin"))
        .and(warp::path("update-password"))
        .and(warp::path::end())
        .and(warp::put())
        .and(store_filter.clone())
        .and(auth(ADMIN_ROLE_ID))
        .and(warp::body::json())
        .and_then(update_password);

    //PUT api/v1/user/deleteUser
    let delete_user_api = user_path
        .and(warp::path("user"))
        .and(warp::path("delete-user"))
        .and(warp::path::end())
        .and(warp::put())
        .and(store_filter.clone())
        .and(auth(USER_ROLE_ID))
        .and(warp::body::json())
        .and_then(delete);

    //PUT api/v1/user/deleteUser
    let delete_hr_api = user_path
        .and(warp::path("user"))
        .and(warp::path("delete-user"))
        .and(warp::path::end())
        .and(warp::put())
        .and(store_filter.clone())
        .and(auth(HR_ROLE_ID))
        .and(warp::body::json())
        .and_then(delete);

    //PUT api/v1/admin/deleteAdmin
    let delete_admin_api = user_path
        .and(warp::path("admin"))
        .and(warp::path("delete-admin"))
        .and(warp::path::end())
        .and(warp::put())
        .and(store_filter.clone())
        .and(auth(ADMIN_ROLE_ID))
        .and(warp::body::json())
        .and_then(delete);

    //PUT api/v1/admin/setHr
    let set_hr_api = user_path
        .and(warp::path("admin"))
        .and(warp::path("set-hr"))
        .and(warp::path::end())
        .and(warp::put())
        .and(store_filter.clone())
        .and(auth(ADMIN_ROLE_ID))
        .and(warp::body::json())
        .and_then(set_hr_role);

    //PUT api/v1/admin/setAdmin
    let set_admin_api = user_path
        .and(warp::path("admin"))
        .and(warp::path("set-admin"))
        .and(warp::path::end())
        .and(warp::put())
        .and(store_filter.clone())
        .and(auth(ADMIN_ROLE_ID))
        .and(warp::body::json())
        .and_then(set_admin_role);

    register_api
        .or(login_api)
        .or(get_list_user_api)
        .or(get_user_api)
        .or(update_user_api)
        .or(update_user_password_api)
        .or(delete_user_api)
        .or(update_hr_api)
        .or(update_hr_password_api)
        .or(delete_hr_api)
        .or(set_hr_api)
        .or(update_admin_api)
        .or(update_admin_password_api)
        .or(delete_admin_api)
        .or(set_admin_api)
}
