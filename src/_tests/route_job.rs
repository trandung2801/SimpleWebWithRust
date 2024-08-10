// use std::process::Command;
// use std::io::{self, Write};
// use futures_util::FutureExt;
// use crate::config::config::{Config};
// use crate::models::store::{StoreActionBasic};
// use crate::{oneshot, setup_store};
// use crate::middleware::convert_to_json::PayloadForLogin;
// use crate::models::user::{AuthInfo};
//
// #[tokio::test]
// async fn job_route_test() -> Result<(), handle_errors::Error>
// {
//     let config_env = Config::new().expect("Config env not set");
//
//     let store = setup_store(&config_env).await;
//
//     let handler = oneshot(store, "127.0.0.1:3032".to_string()).await;
//
//     let login_info = AuthInfo {
//         email: "hr2@gmail.com".to_string(),
//         password: "123456".to_string()
//     };
//
//     print!("Running test job route: POST login ...");
//     let access_token: String;
//     match std::panic::AssertUnwindSafe(login_test(&login_info)).catch_unwind().await {
//         Ok(token) => {
//             access_token = token;
//             println!("✓")
//         },
//         Err(_) => {
//             let _ = handler.sender.send(1);
//             std::process::exit(1);
//         }
//     };
//
//     print!("Running test job route: GET job ...");
//     match std::panic::AssertUnwindSafe(get_job_test()).catch_unwind().await {
//         Ok(_) => println!("✓"),
//         Err(_) => {
//             let _ = handler.sender.send(1);
//             std::process::exit(1);
//         }
//     };
//
//     print!("Running test job route: GET list job ...");
//     match std::panic::AssertUnwindSafe(get_list_job_test()).catch_unwind().await {
//         Ok(_) => println!("✓"),
//         Err(_) => {
//             let _ = handler.sender.send(1);
//             std::process::exit(1);
//         }
//     };
//
//     print!("Running test job route: Put update job ...");
//     let job =
//     match std::panic::AssertUnwindSafe(update_job_test(&access_token, &job)).catch_unwind().await {
//         Ok(_) => println!("✓"),
//         Err(_) => {
//             let _ = handler.sender.send(1);
//             std::process::exit(1);
//         }
//     };
//
//     //delete user test
//     print!("Running test job route: Put delete job ...");
//     match std::panic::AssertUnwindSafe(delete_job_test(&access_token, &job)).catch_unwind().await {
//         Ok(_) => println!("✓"),
//         Err(_) => {
//             let _ = handler.sender.send(1);
//             std::process::exit(1);
//         }
//     };
//
//     let _ = handler.sender.send(1);
//     Ok(())
// }
//
// async fn login_test(new_user: &AuthInfo)  -> String{
//     let client = reqwest::Client::new();
//     let res = client
//         .post("http://localhost:3031/api/v1/login")
//         .json(&new_user)
//         .send()
//         .await
//         .unwrap();
//
//     assert_eq!(res.status(), 200);
//     res
//         .json::<PayloadForLogin>()
//         .await
//         .unwrap()
//         .access_token
// }
//
// async fn get_job_test() {
//     let client = reqwest::Client::new();
//     let res = client
//         .get("http://localhost:3031/api/v1/job/getjob/1")
//         .send()
//         .await
//         .unwrap();
//     assert_eq!(res.status(), 200);
// }
//
// async fn get_list_job_test() {
//     let client = reqwest::Client::new();
//     let res = client
//         .get("http://localhost:3031/api/v1/job/listjob")
//         .send()
//         .await
//         .unwrap();
//     assert_eq!(res.status(), 200);
// }
//
//
// async fn update_job_test(access_token: &String, job: &job) {
//     let client = reqwest::Client::new();
//     let res = client
//         .put("http://localhost:3031/api/v1/job/updatejob")
//         .header("Authorization", format!("Bearer{}", access_token))
//         .json(&job)
//         .send()
//         .await
//         .unwrap();
//     assert_eq!(res.status(), 200);
// }
//
// async fn delete_job_test(access_token: &String, job: &job) {
//     let client = reqwest::Client::new();
//     let res = client
//         .put("http://localhost:3031/api/v1/job/deletejob")
//         .header("Authorization", format!("Bearer{}", access_token))
//         .json(&job)
//         .send()
//         .await
//         .unwrap();
//     assert_eq!(res.status(), 200);
// }
//
