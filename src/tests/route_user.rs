use crate::config::config::{Config};
use crate::models::user::{AuthInfo, UserId, UserInfo};
use crate::{oneshot, setup_store};
use futures_util::future::FutureExt;
use crate::middleware::convert_to_json::{PayloadForLogin};
use crate::models::company::CompanyId;
use crate::models::role::{ADMIN_ROLE_ID, HR_ROLE_ID, RoleId, USER_ROLE_ID};

#[tokio::test]
async fn user_route_test() -> Result<(), handle_errors::Error>
{
    let config_env = Config::new().expect("Config env not set");

    let store = setup_store(&config_env).await;

    let handler = oneshot(store, "127.0.0.1:3030".to_string()).await;

    let new_user = AuthInfo{
        email: "123321@gmail.com".to_string(),
        password: "123456".to_string(),
    };

    print!("Running test user route: POST register success...");
    match std::panic::AssertUnwindSafe(register_success_test(&new_user)).catch_unwind().await {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    };

    print!("Running test user route: POST register false...");
    match std::panic::AssertUnwindSafe(register_false_test(&new_user)).catch_unwind().await {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    };

    //For User
    print!("Running test user route: POST login ...");
    let access_token_user: String;
    match std::panic::AssertUnwindSafe(login_test(&new_user)).catch_unwind().await {
        Ok(token) => {
            access_token_user = token;
            println!("✓")
        },
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    };

    print!("Running test user route: GET user ...");
    match std::panic::AssertUnwindSafe(get_user_test()).catch_unwind().await {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    };

    print!("Running test user route: GET list user ...");
    match std::panic::AssertUnwindSafe(get_list_user_test()).catch_unwind().await {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    };

    print!("Running test user route: Put update user ...");
    let user_info = UserInfo {
        id: UserId(9),
        email: "123321@gmail.com".to_string(),
        company_id: CompanyId(1),
        role_id: RoleId(USER_ROLE_ID),
        is_delete: false,
    };
    match std::panic::AssertUnwindSafe(update_user_test(&access_token_user, &user_info)).catch_unwind().await {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    };

    print!("Running test user route: Put update password user ...");
    let user_update_pass = AuthInfo {
        email: "123321@gmail.com".to_string(),
        password: "123456789".to_string(),
    };
    match std::panic::AssertUnwindSafe(update_password_user_test(&access_token_user, &user_update_pass)).catch_unwind().await {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    };

    //For Hr
    print!("Running test user route: POST hr login ...");
    let access_token_hr: String;
    let hr_login = AuthInfo {
        email: "hr1@gmail.com".to_string(),
        password: "123456".to_string()
    };
    match std::panic::AssertUnwindSafe(login_test(&hr_login)).catch_unwind().await {
        Ok(token) => {
            access_token_hr = token;
            println!("✓")
        },
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    };

    print!("Running test user route: Put update hr ...");
    let hr_info = UserInfo {
        id: UserId(3),
        email: "hr1@gmail.com".to_string(),
        company_id: CompanyId(2),
        role_id: RoleId(HR_ROLE_ID),
        is_delete: false,
    };
    match std::panic::AssertUnwindSafe(update_user_test(&access_token_hr, &hr_info)).catch_unwind().await {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    };

    print!("Running test user route: Put update password hr ...");
    let hr_update_pass = AuthInfo {
        email: "hr1@gmail.com".to_string(),
        password: "123456789".to_string(),
    };
    match std::panic::AssertUnwindSafe(update_password_user_test(&access_token_hr, &hr_update_pass)).catch_unwind().await {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    };



    //For admin
    print!("Running test user route: POST admin login ...");
    let access_token_admin: String;
    let admin_login = AuthInfo {
        email: "admin1@gmail.com".to_string(),
        password: "123456".to_string()
    };
    match std::panic::AssertUnwindSafe(login_test(&admin_login)).catch_unwind().await {
        Ok(token) => {
            access_token_admin = token;
            println!("✓")
        },
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    };

    print!("Running test user route: Put update admin ...");
    let admin_info = UserInfo {
        id: UserId(2),
        email: "admin1@gmail.com".to_string(),
        company_id: CompanyId(0),
        role_id: RoleId(ADMIN_ROLE_ID),
        is_delete: false,
    };
    match std::panic::AssertUnwindSafe(update_admin_test(&access_token_admin, &admin_info)).catch_unwind().await {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    };

    print!("Running test user route: Put update password admin ...");
    let admin_update_pass = AuthInfo {
        email: "admin1@gmail.com".to_string(),
        password: "123456789".to_string(),
    };
    match std::panic::AssertUnwindSafe(update_password_admin_test(&access_token_admin, &admin_update_pass)).catch_unwind().await {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    };

    print!("Running test user route: Put set hr ...");
    match std::panic::AssertUnwindSafe(set_hr_test(&access_token_admin, &user_info)).catch_unwind().await {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    };

    print!("Running test user route: Put set admin ...");
    match std::panic::AssertUnwindSafe(set_admin_test(&access_token_admin, &user_info)).catch_unwind().await {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    };

    //delete user test
    print!("Running test user route: Put delete user ...");
    match std::panic::AssertUnwindSafe(delete_user_test(&access_token_user, &user_info)).catch_unwind().await {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    };

    print!("Running test user route: Put delete hr ...");
    match std::panic::AssertUnwindSafe(delete_user_test(&access_token_hr, &hr_info)).catch_unwind().await {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    };

    print!("Running test user route: Put delete admin ...");
    match std::panic::AssertUnwindSafe(delete_admin_test(&access_token_admin, &admin_info)).catch_unwind().await {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    };

    let _ = handler.sender.send(1);
    Ok(())
}

async fn register_success_test(new_user: &AuthInfo)  {
    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:3030/api/v1/register")
        .json(&new_user)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 201);
}

async fn register_false_test(new_user: &AuthInfo)  {
    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:3030/api/v1/register")
        .json(&new_user)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 400);
}

async fn login_test(new_user: &AuthInfo)  -> String{
    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:3030/api/v1/login")
        .json(&new_user)
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 200);
    res
        .json::<PayloadForLogin>()
        .await
        .unwrap()
        .access_token
}

async fn get_user_test() {
    let client = reqwest::Client::new();
    let res = client
        .get("http://localhost:3030/api/v1/user/getUser/1")
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
}

async fn get_list_user_test() {
    let client = reqwest::Client::new();
    let res = client
        .get("http://localhost:3030/api/v1/user/listUser")
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
}

async fn update_user_test(access_token: &String, user_info: &UserInfo) {
    let client = reqwest::Client::new();
    let res = client
        .put("http://localhost:3030/api/v1/user/updateUser")
        .header("Authorization", format!("Bearer{}", access_token))
        .json(&user_info)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
}

async fn update_admin_test(access_token: &String, user_info: &UserInfo) {
    let client = reqwest::Client::new();
    let res = client
        .put("http://localhost:3030/api/v1/admin/updateAdmin")
        .header("Authorization", format!("Bearer{}", access_token))
        .json(&user_info)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
}

async fn update_password_user_test(access_token: &String, user_info: &AuthInfo) {
    let client = reqwest::Client::new();
    let res = client
        .put("http://localhost:3030/api/v1/user/updatePassword")
        .header("Authorization", format!("Bearer{}", access_token))
        .json(&user_info)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
}

async fn update_password_admin_test(access_token: &String, user_info: &AuthInfo) {
    let client = reqwest::Client::new();
    let res = client
        .put("http://localhost:3030/api/v1/admin/updatePassword")
        .header("Authorization", format!("Bearer{}", access_token))
        .json(&user_info)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
}

async fn delete_user_test(access_token: &String, user_info: &UserInfo) {
    let client = reqwest::Client::new();
    let res = client
        .put("http://localhost:3030/api/v1/user/deleteUser")
        .header("Authorization", format!("Bearer{}", access_token))
        .json(&user_info)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
}

async fn delete_admin_test(access_token: &String, user_info: &UserInfo) {
    let client = reqwest::Client::new();
    let res = client
        .put("http://localhost:3030/api/v1/admin/deleteAdmin")
        .header("Authorization", format!("Bearer{}", access_token))
        .json(&user_info)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
}

async fn set_hr_test(access_token: &String, user_info: &UserInfo) {
    let client = reqwest::Client::new();
    let res = client
        .put("http://localhost:3030/api/v1/admin/setHr")
        .header("Authorization", format!("Bearer{}", access_token))
        .json(&user_info)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
}

async fn set_admin_test(access_token: &String, user_info: &UserInfo) {
    let client = reqwest::Client::new();
    let res = client
        .put("http://localhost:3030/api/v1/admin/setAdmin")
        .header("Authorization", format!("Bearer{}", access_token))
        .json(&user_info)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
}