use futures_util::FutureExt;
use crate::config::config::{Config};
use crate::{oneshot, setup_store};
use crate::middleware::convert_to_json::PayloadForLogin;
use crate::models::company::CompanyId;
use crate::models::job::JobId;
use crate::models::resume::{Resume, ResumeId, NewResume};
use crate::models::user::{AuthInfo, UserId};

#[tokio::test]
async fn resume_route_test() -> Result<(), handle_errors::Error>
{
    let config_env = Config::new().expect("Config env not set");

    let store = setup_store(&config_env).await;

    let handler = oneshot(store, "127.0.0.1:3033".to_string()).await;

    let login_info = AuthInfo {
        email: "user2@gmail.com".to_string(),
        password: "123456".to_string()
    };

    print!("Running test resume route: POST login ...");
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

    print!("Running test resume route: GET resume ...");
    match std::panic::AssertUnwindSafe(get_resume_test(&access_token)).catch_unwind().await {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    };

    print!("Running test resume route: GET list resume by user ...");
    match std::panic::AssertUnwindSafe(get_list_resume_user_test(&access_token)).catch_unwind().await {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    };

    print!("Running test resume route: GET list resume by job ...");
    match std::panic::AssertUnwindSafe(get_list_resume_job_test(JobId(1))).catch_unwind().await {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    };

    print!("Running test resume route: POST create resume ...");
    let new_resume = NewResume{
        user_id: UserId(7),
        email: "user2@gmail.com".to_string(),
        url: "dsadasdasdasdjaslkjda".to_string()
    };
    match std::panic::AssertUnwindSafe(create_resume_test(&access_token, &new_resume)).catch_unwind().await {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    };

    print!("Running test resume route: Put update resume ...");
    let resume = Resume {
        id: Some(ResumeId(1)),
        user_id: UserId(7),
        email: "user2@gmail.com".to_string(),
        url: "jqka".to_string(),
        is_delete: false
    };
    match std::panic::AssertUnwindSafe(update_resume_test(&access_token, &resume)).catch_unwind().await {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    };

    //delete user test
    print!("Running test resume route: Put delete resume ...");
    match std::panic::AssertUnwindSafe(delete_resume_test(&access_token, &resume)).catch_unwind().await {
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
        .post("http://localhost:3033/api/v1/login")
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

async fn get_resume_test(access_token: &String) {
    let client = reqwest::Client::new();
    let res = client
        .get("http://localhost:3033/api/v1/resume/getResume/1")
        .header("Authorization", format!("Bearer{}", access_token))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
}

async fn get_list_resume_user_test(access_token: &String) {
    let client = reqwest::Client::new();
    let res = client
        .get("http://localhost:3033/api/v1/resume/listResumeByUser?limit=10&offset=0")
        .header("Authorization", format!("Bearer{}", access_token))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
}


async fn get_list_resume_job_test(job_id: JobId) {
    let client = reqwest::Client::new();
    let res = client
        .get(format!("http://localhost:3033/api/v1/resume/listResumeByJob?limit=10&offset=0&jobId={}", job_id.0))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
}

async fn create_resume_test(access_token: &String, new_resume: &NewResume) {
    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:3033/api/v1/resume/createResume")
        .header("Authorization", format!("Bearer{}", access_token))
        .json(&new_resume)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 201);
}


async fn update_resume_test(access_token: &String, resume: &Resume) {
    let client = reqwest::Client::new();
    let res = client
        .put("http://localhost:3033/api/v1/resume/updateResume")
        .header("Authorization", format!("Bearer{}", access_token))
        .json(&resume)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
}

async fn delete_resume_test(access_token: &String, resume: &Resume) {
    let client = reqwest::Client::new();
    let res = client
        .put("http://localhost:3033/api/v1/resume/deleteResume")
        .header("Authorization", format!("Bearer{}", access_token))
        .json(&resume)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
}

