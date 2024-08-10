use std::process::Command;
use std::io::{self, Write};
use futures_util::FutureExt;
use crate::config::config::{Config};
use crate::models::store::{StoreActionBasic};
use crate::{oneshot, setup_store};
use crate::middleware::convert_to_json::PayloadForLogin;
use crate::models::company::{Company, CompanyId};
use crate::models::user::{AuthInfo};

#[tokio::test]
async fn company_route_test() -> Result<(), handle_errors::Error>
{
    let config_env = Config::new().expect("Config env not set");

    let store = setup_store(&config_env).await;

    let handler = oneshot(store, "127.0.0.1:3031".to_string()).await;

    let login_info = AuthInfo {
        email: "admin@gmail.com".to_string(),
        password: "123456".to_string()
    };

    print!("Running test company route: POST login ...");
    let access_token: String;
    match std::panic::AssertUnwindSafe(login_test(&login_info)).catch_unwind().await {
        Ok(token) => {
            access_token = token;
            println!("✓")
        },
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    };

    print!("Running test company route: GET company ...");
    match std::panic::AssertUnwindSafe(get_company_test()).catch_unwind().await {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    };

    print!("Running test company route: GET list company ...");
    match std::panic::AssertUnwindSafe(get_list_company_test()).catch_unwind().await {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    };

    print!("Running test company route: Put update company ...");
    let company = Company {
        id: Some(CompanyId(1)),
        name: "Sotanextnerxt".to_string(),
        email: "sotanextnext@gmail.com".to_string(),
        address: "Tang 5 Golden Park So 2 Pham Van Bach bach".to_string(),
        description: "Company out source for blockchain and web 3".to_string(),
        is_delete: false
    };
    match std::panic::AssertUnwindSafe(update_company_test(&access_token, &company)).catch_unwind().await {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    };

    //delete user test
    print!("Running test company route: Put delete company ...");
    match std::panic::AssertUnwindSafe(delete_company_test(&access_token, &company)).catch_unwind().await {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    };

    let _ = handler.sender.send(1);
    Ok(())
}

async fn login_test(new_user: &AuthInfo)  -> String{
    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:3031/api/v1/login")
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

async fn get_company_test() {
    let client = reqwest::Client::new();
    let res = client
        .get("http://localhost:3031/api/v1/company/getCompany/1")
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
}

async fn get_list_company_test() {
    let client = reqwest::Client::new();
    let res = client
        .get("http://localhost:3031/api/v1/company/listCompany")
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
}


async fn update_company_test(access_token: &String, company: &Company) {
    let client = reqwest::Client::new();
    let res = client
        .put("http://localhost:3031/api/v1/company/updateCompany")
        .header("Authorization", format!("Bearer{}", access_token))
        .json(&company)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
}

async fn delete_company_test(access_token: &String, company: &Company) {
    let client = reqwest::Client::new();
    let res = client
        .put("http://localhost:3031/api/v1/company/deleteCompany")
        .header("Authorization", format!("Bearer{}", access_token))
        .json(&company)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
}

