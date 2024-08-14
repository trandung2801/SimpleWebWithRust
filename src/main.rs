use std::sync::Arc;
use tracing_subscriber::fmt::format::FmtSpan;
use warp::{http::Method, Filter};
use tokio::sync::{oneshot, oneshot::Sender};
use tracing::{instrument};
use handle_errors::return_error;
use routes::user::user_route;
use crate::config::config::Config;
use crate::models::store_db::Store;
use crate::models::store_trait::StoreMethods;
use crate::routes::company::company_route;
use crate::routes::job::job_route;
use crate::routes::resume::resume_route;

mod models;
mod routes;
mod controllers;
mod config;
mod middleware;

#[tokio::main]
#[instrument]
async fn main() {
    let config = Config::new().expect("Config env not set");

    // let store: Arc<dyn StoreMethods> = if config.database.is_none() || config.database.clone().unwrap() == "in-memory".to_string() {
    //     // println!("In-memory")
    // } else {
    //     // let x = setup_store(&config).await;
    //     setup_store(&config)
    // };

    // let store = setup_store(&config).await;
    let url = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.postgres.db_user,
        config.postgres.db_password,
        config.postgres.db_host,
        config.postgres.db_port,
        config.postgres.db_name
    );

    // let store: Arc<dyn StoreMethods> = Arc::new(Store::new(&url));
    let store: Arc<dyn StoreMethods + Send + Sync> = Arc::new(Store::new(&url).await);


    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[
            Method::PUT,
            Method::DELETE,
            Method::GET,
            Method::POST,
        ]);

    let log_filter = format!(
        "handle_errors={},backend={},warp={}",
        config.log_level, config.log_level, config.log_level
    );

    tracing_subscriber::fmt()
        // Use the filter we built above to determine which traces to record.
        .with_env_filter(log_filter)
        // Record an event when each span closes. This can be used to time our
        // routes' durations!
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let user_routes = user_route("api", store.clone());
    let company_routes = company_route("api", store.clone());
    let resume_routes = resume_route("api", store.clone());
    let job_routes = job_route("api", store.clone());
    let routes = user_routes
        .or(company_routes)
        .or(resume_routes)
        .or(job_routes)
        .with(cors)
        .with(warp::trace::request())
        .recover(return_error);

    let address_listen = format!("{}:{}", config.server.host, config.server.port);
    let socket: std::net::SocketAddr = address_listen
        .parse()
        .expect("Not a valid address");
    // warp::serve(routes).run(([127, 0, 0, 1], config.port)).await;
    warp::serve(routes).run(socket).await;
}


pub async fn setup_store(config: &Config) -> Store {
    let store: Store = Store::new(&format!(
        "postgres://{}:{}@{}:{}/{}",
        config.postgres.db_user,
        config.postgres.db_password,
        config.postgres.db_host,
        config.postgres.db_port,
        config.postgres.db_name
    )).await;

    store

}

pub struct OneshotHandler {
    pub sender: Sender<i32>
}
// pub async fn oneshot(store: Store, address_listen: String) -> OneshotHandler
// {
//     let cors = warp::cors()
//         .allow_any_origin()
//         .allow_header("content-type")
//         .allow_methods(&[
//             Method::PUT,
//             Method::DELETE,
//             Method::GET,
//             Method::POST,
//         ]);
//
//     let user_routes = user_route("api", store.clone());
//     let company_routes = company_route("api", store.clone());
//     let resume_routes = resume_route("api", store.clone());
//     let job_routes = job_route("api", store.clone());
//     let routes = user_routes
//         .or(company_routes)
//         .or(resume_routes)
//         .or(job_routes)
//         .with(cors)
//         .with(warp::trace::request())
//         .recover(return_error);
//
//     let (tx, rx) = oneshot::channel::<i32>();
//     let socket: std::net::SocketAddr = address_listen
//         .parse()
//         .expect("Not a valid address");
//
//     let (_, server) = warp::serve(routes).bind_with_graceful_shutdown(socket, async {
//         rx.await.ok();
//     });
//     tokio::task::spawn(server);
//     OneshotHandler{sender:tx}
// }
