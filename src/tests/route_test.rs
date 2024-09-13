use std::string::ToString;

use futures_util::FutureExt;

use crate::models::company::{Company, CompanyId, NewCompany};
use crate::models::job::{Job, JobId, NewJob};
use crate::models::resume::{NewResume, Resume, ResumeId};
use crate::models::role::{RoleId, ADMIN_ROLE_ID, HR_ROLE_ID, USER_ROLE_ID};
use crate::models::user::{AuthInfo, UserId, UserInfo};
use crate::utils::convert_to_json::{Data, PayloadForLogin, PayloadWithData};
use crate::{build_store_for_test, init_test_server};

#[tokio::test]
async fn route_test() {
    let server_host: String = "0.0.0.0".to_string();
    let server_port: u16 = 3030;
    let url: String = "postgres://postgres:123456@127.0.0.1:5432/postgres".to_string();
    let database: String = "postgres".to_string();
    let sample_data_url: String = "migrations/postgres/sample.sql".to_string();

    let address_listen = format!("{}:{}", server_host, server_port);
    let store = build_store_for_test(url, database, sample_data_url).await;
    let handler = init_test_server(address_listen, store).await;

    let new_user = AuthInfo {
        email: "123321@gmail.com".to_string(),
        hash_password: "123456".to_string(),
    };

    print!("Running test user route: POST register success...");
    match std::panic::AssertUnwindSafe(register_success_test(&new_user))
        .catch_unwind()
        .await
    {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.send(1);
            std::process::exit(1);
        }
    };

    //For User
    print!("Running test user route: POST login ...");
    let access_token_user: String;
    match std::panic::AssertUnwindSafe(login_test(&new_user))
        .catch_unwind()
        .await
    {
        Ok(token) => {
            access_token_user = token;
            println!("✓")
        }
        Err(_) => {
            let _ = handler.send(1);
            std::process::exit(1);
        }
    };

    print!("Running test user route: GET user ...");
    let expect_get_user = UserInfo {
        id: UserId(1),
        email: "admin@gmail.com".to_string(),
        company_id: CompanyId(0),
        role_id: RoleId(1),
        is_delete: false,
    };
    match std::panic::AssertUnwindSafe(get_user_test(expect_get_user))
        .catch_unwind()
        .await
    {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.send(1);
            std::process::exit(1);
        }
    };

    print!("Running test user route: GET list user ...");
    match std::panic::AssertUnwindSafe(get_list_user_test())
        .catch_unwind()
        .await
    {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.send(1);
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
    match std::panic::AssertUnwindSafe(update_user_test(&access_token_user, &user_info))
        .catch_unwind()
        .await
    {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.send(1);
            std::process::exit(1);
        }
    };

    print!("Running test user route: Put update password user ...");
    let user_update_pass = AuthInfo {
        email: "123321@gmail.com".to_string(),
        hash_password: "123456789".to_string(),
    };
    match std::panic::AssertUnwindSafe(update_password_user_test(
        &access_token_user,
        &user_update_pass,
    ))
    .catch_unwind()
    .await
    {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.send(1);
            std::process::exit(1);
        }
    };

    //For Hr
    print!("Running test user route: POST hr login ...");
    let access_token_hr: String;
    let hr_login = AuthInfo {
        email: "hr1@gmail.com".to_string(),
        hash_password: "123456".to_string(),
    };
    match std::panic::AssertUnwindSafe(login_test(&hr_login))
        .catch_unwind()
        .await
    {
        Ok(token) => {
            access_token_hr = token;
            println!("✓")
        }
        Err(_) => {
            let _ = handler.send(1);
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
    match std::panic::AssertUnwindSafe(update_user_test(&access_token_hr, &hr_info))
        .catch_unwind()
        .await
    {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.send(1);
            std::process::exit(1);
        }
    };

    print!("Running test user route: Put update password hr ...");
    let hr_update_pass = AuthInfo {
        email: "hr1@gmail.com".to_string(),
        hash_password: "123456789".to_string(),
    };
    match std::panic::AssertUnwindSafe(update_password_user_test(&access_token_hr, &hr_update_pass))
        .catch_unwind()
        .await
    {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.send(1);
            std::process::exit(1);
        }
    };

    //For admin
    print!("Running test user route: POST admin login ...");
    let access_token_admin: String;
    let admin_login = AuthInfo {
        email: "admin1@gmail.com".to_string(),
        hash_password: "123456".to_string(),
    };
    match std::panic::AssertUnwindSafe(login_test(&admin_login))
        .catch_unwind()
        .await
    {
        Ok(token) => {
            access_token_admin = token;
            println!("✓")
        }
        Err(_) => {
            let _ = handler.send(1);
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
    match std::panic::AssertUnwindSafe(update_admin_test(&access_token_admin, &admin_info))
        .catch_unwind()
        .await
    {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.send(1);
            std::process::exit(1);
        }
    };

    print!("Running test user route: Put update password admin ...");
    let admin_update_pass = AuthInfo {
        email: "admin1@gmail.com".to_string(),
        hash_password: "123456789".to_string(),
    };
    match std::panic::AssertUnwindSafe(update_password_admin_test(
        &access_token_admin,
        &admin_update_pass,
    ))
    .catch_unwind()
    .await
    {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.send(1);
            std::process::exit(1);
        }
    };

    print!("Running test user route: Put set hr ...");
    match std::panic::AssertUnwindSafe(set_hr_test(&access_token_admin, &user_info))
        .catch_unwind()
        .await
    {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.send(1);
            std::process::exit(1);
        }
    };

    print!("Running test user route: Put set admin ...");
    match std::panic::AssertUnwindSafe(set_admin_test(&access_token_admin, &user_info))
        .catch_unwind()
        .await
    {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.send(1);
            std::process::exit(1);
        }
    };

    //delete user test
    print!("Running test user route: Put delete user ...");
    match std::panic::AssertUnwindSafe(delete_user_test(&access_token_user, &user_info))
        .catch_unwind()
        .await
    {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.send(1);
            std::process::exit(1);
        }
    };

    print!("Running test user route: Put delete hr ...");
    match std::panic::AssertUnwindSafe(delete_user_test(&access_token_hr, &hr_info))
        .catch_unwind()
        .await
    {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.send(1);
            std::process::exit(1);
        }
    };

    print!("Running test user route: Put delete admin ...");
    match std::panic::AssertUnwindSafe(delete_admin_test(&access_token_admin, &admin_info))
        .catch_unwind()
        .await
    {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.send(1);
            std::process::exit(1);
        }
    };

    // For company route test
    let login_test_company = AuthInfo {
        email: "admin@gmail.com".to_string(),
        hash_password: "123456".to_string(),
    };

    print!("Running test company route: POST login ...");
    let access_token_company: String;
    match std::panic::AssertUnwindSafe(login_test(&login_test_company))
        .catch_unwind()
        .await
    {
        Ok(token) => {
            access_token_company = token;
            println!("✓")
        }
        Err(_) => {
            let _ = handler.send(1);
            std::process::exit(1);
        }
    };

    print!("Running test company route: GET company ...");
    match std::panic::AssertUnwindSafe(get_company_test())
        .catch_unwind()
        .await
    {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.send(1);
            std::process::exit(1);
        }
    };

    print!("Running test company route: GET list company ...");
    match std::panic::AssertUnwindSafe(get_list_company_test())
        .catch_unwind()
        .await
    {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.send(1);
            std::process::exit(1);
        }
    };

    print!("Running test company route: POST create company ...");
    let new_company = NewCompany {
        email: "sotatek999@gmail.com".to_string(),
        name: "Sotatek999".to_string(),
        address: "2 Pham Van Bach".to_string(),
        description: "Company out source for blockchain and web 3".to_string(),
    };
    match std::panic::AssertUnwindSafe(create_company_test(&access_token_company, &new_company))
        .catch_unwind()
        .await
    {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.send(1);
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
        is_delete: false,
    };
    match std::panic::AssertUnwindSafe(update_company_test(&access_token_company, &company))
        .catch_unwind()
        .await
    {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.send(1);
            std::process::exit(1);
        }
    };

    print!("Running test company route: Put delete company ...");
    match std::panic::AssertUnwindSafe(delete_company_test(&access_token_company, &company))
        .catch_unwind()
        .await
    {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.send(1);
            std::process::exit(1);
        }
    };

    //For resume route test
    let login_test_resume = AuthInfo {
        email: "user2@gmail.com".to_string(),
        hash_password: "123456".to_string(),
    };

    print!("Running test resume route: POST login ...");
    let access_token_resume: String;
    match std::panic::AssertUnwindSafe(login_test(&login_test_resume))
        .catch_unwind()
        .await
    {
        Ok(token) => {
            access_token_resume = token;
            println!("✓")
        }
        Err(_) => {
            let _ = handler.send(1);
            std::process::exit(1);
        }
    };

    print!("Running test resume route: GET resume ...");
    let expect_get_resume = Resume {
        id: Some(ResumeId(1)),
        user_id: UserId(7),
        email: "user2@gmail.com".to_string(),
        url: "abcxyz".to_string(),
        is_delete: false,
    };
    match std::panic::AssertUnwindSafe(get_resume_test(&access_token_resume, expect_get_resume))
        .catch_unwind()
        .await
    {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.send(1);
            std::process::exit(1);
        }
    };

    print!("Running test resume route: GET list resume by user ...");
    match std::panic::AssertUnwindSafe(get_list_resume_user_test(&access_token_resume))
        .catch_unwind()
        .await
    {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.send(1);
            std::process::exit(1);
        }
    };

    print!("Running test resume route: GET list resume by job ...");
    match std::panic::AssertUnwindSafe(get_list_resume_job_test(JobId(1)))
        .catch_unwind()
        .await
    {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.send(1);
            std::process::exit(1);
        }
    };

    print!("Running test resume route: POST create resume ...");
    let new_resume = NewResume {
        user_id: UserId(7),
        email: "user2@gmail.com".to_string(),
        url: "dsadasdasdasdjaslkjda".to_string(),
    };
    match std::panic::AssertUnwindSafe(create_resume_test(&access_token_resume, &new_resume))
        .catch_unwind()
        .await
    {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.send(1);
            std::process::exit(1);
        }
    };

    print!("Running test resume route: Put update resume ...");
    let resume = Resume {
        id: Some(ResumeId(1)),
        user_id: UserId(7),
        email: "user2@gmail.com".to_string(),
        url: "jqka".to_string(),
        is_delete: false,
    };
    match std::panic::AssertUnwindSafe(update_resume_test(&access_token_resume, &resume))
        .catch_unwind()
        .await
    {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.send(1);
            std::process::exit(1);
        }
    };

    print!("Running test resume route: Put delete resume ...");
    match std::panic::AssertUnwindSafe(delete_resume_test(&access_token_resume, &resume))
        .catch_unwind()
        .await
    {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.send(1);
            std::process::exit(1);
        }
    };

    let login_test_job = AuthInfo {
        email: "hr2@gmail.com".to_string(),
        hash_password: "123456".to_string(),
    };

    print!("Running test job route: POST login ...");
    let access_token_job: String;
    match std::panic::AssertUnwindSafe(login_test(&login_test_job))
        .catch_unwind()
        .await
    {
        Ok(token) => {
            access_token_job = token;
            println!("✓")
        }
        Err(_) => {
            let _ = handler.send(1);
            std::process::exit(1);
        }
    };

    print!("Running test job route: GET job ...");
    match std::panic::AssertUnwindSafe(get_job_test())
        .catch_unwind()
        .await
    {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.send(1);
            std::process::exit(1);
        }
    };

    print!("Running test job route: GET list job ...");
    match std::panic::AssertUnwindSafe(get_list_job_test())
        .catch_unwind()
        .await
    {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.send(1);
            std::process::exit(1);
        }
    };

    print!("Running test job route: POST create job ...");
    let new_job = NewJob {
        job_name: "Junior Web3".to_string(),
        company_id: CompanyId(1),
        location: "So 2 Pham Van Bach".to_string(),
        quantity: 3,
        salary: 7_000_000,
        job_level: "Junior".to_string(),
        description: "Junior of web 3 has knowledge rust".to_string(),
    };
    match std::panic::AssertUnwindSafe(create_job_test(&access_token_job, &new_job))
        .catch_unwind()
        .await
    {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.send(1);
            std::process::exit(1);
        }
    };

    print!("Running test job route: Put update job ...");
    let job = Job {
        id: Some(JobId(2)),
        job_name: "Junior Web3".to_string(),
        company_id: CompanyId(1),
        location: "So 2 Pham Van Bach".to_string(),
        quantity: 1,
        salary: 12_000_000,
        job_level: "Junior".to_string(),
        description: "Junior from UET has knowledge rust".to_string(),
        is_delete: false,
    };
    match std::panic::AssertUnwindSafe(update_job_test(&access_token_job, &job))
        .catch_unwind()
        .await
    {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.send(1);
            std::process::exit(1);
        }
    };

    print!("Running test job route: Put delete job ...");
    match std::panic::AssertUnwindSafe(delete_job_test(&access_token_job, &job))
        .catch_unwind()
        .await
    {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.send(1);
            std::process::exit(1);
        }
    };

    let _ = handler.send(1);
}

pub async fn register_success_test(new_user: &AuthInfo) {
    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:3030/api/v1/register")
        .json(&new_user)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 201);
}
pub async fn login_test(new_user: &AuthInfo) -> String {
    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:3030/api/v1/login")
        .json(&new_user)
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 200);
    res.json::<PayloadForLogin>().await.unwrap().access_token
}

// For user
pub async fn get_user_test(expect_data: UserInfo) {
    let client = reqwest::Client::new();
    let res = client
        .get("http://localhost:3030/api/v1/user/get-user/1")
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    assert_eq!(
        res.json::<PayloadWithData>().await.unwrap().data,
        Data::UserInfo(expect_data)
    );
}

pub async fn get_list_user_test() {
    let client = reqwest::Client::new();
    let res = client
        .get("http://localhost:3030/api/v1/user/list-user?limit=10&offset=0")
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
}

pub async fn update_user_test(access_token: &String, user_info: &UserInfo) {
    let client = reqwest::Client::new();
    let res = client
        .put("http://localhost:3030/api/v1/user/update-user")
        .header("Authorization", format!("Bearer{}", access_token))
        .json(&user_info)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
}

pub async fn update_admin_test(access_token: &String, user_info: &UserInfo) {
    let client = reqwest::Client::new();
    let res = client
        .put("http://localhost:3030/api/v1/admin/update-admin")
        .header("Authorization", format!("Bearer{}", access_token))
        .json(&user_info)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
}

pub async fn update_password_user_test(access_token: &String, user_info: &AuthInfo) {
    let client = reqwest::Client::new();
    let res = client
        .put("http://localhost:3030/api/v1/user/update-password")
        .header("Authorization", format!("Bearer{}", access_token))
        .json(&user_info)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
}

pub async fn update_password_admin_test(access_token: &String, user_info: &AuthInfo) {
    let client = reqwest::Client::new();
    let res = client
        .put("http://localhost:3030/api/v1/admin/update-password")
        .header("Authorization", format!("Bearer{}", access_token))
        .json(&user_info)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
}

pub async fn delete_user_test(access_token: &String, user_info: &UserInfo) {
    let client = reqwest::Client::new();
    let res = client
        .put("http://localhost:3030/api/v1/user/delete-user")
        .header("Authorization", format!("Bearer{}", access_token))
        .json(&user_info)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
}

pub async fn delete_admin_test(access_token: &String, user_info: &UserInfo) {
    let client = reqwest::Client::new();
    let res = client
        .put("http://localhost:3030/api/v1/admin/delete-admin")
        .header("Authorization", format!("Bearer{}", access_token))
        .json(&user_info)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
}

pub async fn set_hr_test(access_token: &String, user_info: &UserInfo) {
    let client = reqwest::Client::new();
    let res = client
        .put("http://localhost:3030/api/v1/admin/set-hr")
        .header("Authorization", format!("Bearer{}", access_token))
        .json(&user_info)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
}

pub async fn set_admin_test(access_token: &String, user_info: &UserInfo) {
    let client = reqwest::Client::new();
    let res = client
        .put("http://localhost:3030/api/v1/admin/set-admin")
        .header("Authorization", format!("Bearer{}", access_token))
        .json(&user_info)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
}

//For company route test
pub async fn get_company_test() {
    let client = reqwest::Client::new();
    let res = client
        .get("http://localhost:3030/api/v1/company/get-company/1")
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
}

pub async fn get_list_company_test() {
    let client = reqwest::Client::new();
    let res = client
        .get("http://localhost:3030/api/v1/company/list-company?limit=10&offset=0")
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
}

pub async fn create_company_test(access_token: &String, new_company: &NewCompany) {
    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:3030/api/v1/company/create-company")
        .header("Authorization", format!("Bearer{}", access_token))
        .json(&new_company)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 201);
}

pub async fn update_company_test(access_token: &String, company: &Company) {
    let client = reqwest::Client::new();
    let res = client
        .put("http://localhost:3030/api/v1/company/update-company")
        .header("Authorization", format!("Bearer{}", access_token))
        .json(&company)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
}

pub async fn delete_company_test(access_token: &String, company: &Company) {
    let client = reqwest::Client::new();
    let res = client
        .put("http://localhost:3030/api/v1/company/delete-company")
        .header("Authorization", format!("Bearer{}", access_token))
        .json(&company)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
}

//For job route test
pub async fn get_job_test() {
    let client = reqwest::Client::new();
    let res = client
        .get("http://localhost:3030/api/v1/job/get-job/1")
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
}

pub async fn get_list_job_test() {
    let client = reqwest::Client::new();
    let res = client
        .get("http://localhost:3030/api/v1/job/list-job?limit=10&offset=0")
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
}

pub async fn create_job_test(access_token: &String, new_job: &NewJob) {
    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:3030/api/v1/job/create-job")
        .header("Authorization", format!("Bearer{}", access_token))
        .json(&new_job)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 201);
}

pub async fn update_job_test(access_token: &String, job: &Job) {
    let client = reqwest::Client::new();
    let res = client
        .put("http://localhost:3030/api/v1/job/update-job")
        .header("Authorization", format!("Bearer{}", access_token))
        .json(&job)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
}

pub async fn delete_job_test(access_token: &String, job: &Job) {
    let client = reqwest::Client::new();
    let res = client
        .put("http://localhost:3030/api/v1/job/delete-job")
        .header("Authorization", format!("Bearer{}", access_token))
        .json(&job)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
}

//For resume route test
pub async fn get_resume_test(access_token: &String, expect_data: Resume) {
    let client = reqwest::Client::new();
    let res = client
        .get("http://localhost:3030/api/v1/resume/get-resume/1")
        .header("Authorization", format!("Bearer{}", access_token))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    assert_eq!(
        res.json::<PayloadWithData>().await.unwrap().data,
        Data::Resume(expect_data)
    );
}

pub async fn get_list_resume_user_test(access_token: &String) {
    let client = reqwest::Client::new();
    let res = client
        .get("http://localhost:3030/api/v1/resume/list-resume-by-user?limit=10&offset=0")
        .header("Authorization", format!("Bearer{}", access_token))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
}

pub async fn get_list_resume_job_test(job_id: JobId) {
    let client = reqwest::Client::new();
    let res = client
        .get(format!(
            "http://localhost:3030/api/v1/resume/list-resume-by-job?limit=10&offset=0&jobId={}",
            job_id.0
        ))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
}

pub async fn create_resume_test(access_token: &String, new_resume: &NewResume) {
    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:3030/api/v1/resume/create-resume")
        .header("Authorization", format!("Bearer{}", access_token))
        .json(&new_resume)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 201);
}

pub async fn update_resume_test(access_token: &String, resume: &Resume) {
    let client = reqwest::Client::new();
    let res = client
        .put("http://localhost:3030/api/v1/resume/update-resume")
        .header("Authorization", format!("Bearer{}", access_token))
        .json(&resume)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
}

pub async fn delete_resume_test(access_token: &String, resume: &Resume) {
    let client = reqwest::Client::new();
    let res = client
        .put("http://localhost:3030/api/v1/resume/delete-resume")
        .header("Authorization", format!("Bearer{}", access_token))
        .json(&resume)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
}
