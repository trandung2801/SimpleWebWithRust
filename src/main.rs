use tracing_subscriber::fmt::format::FmtSpan;
use warp::{http::Method, Filter};
use tokio::sync::{oneshot, oneshot::Sender};
use warp::multipart::Part;
use handle_errors::return_error;
use routes::user::user_route;
use crate::config::config::Config;
use crate::models::store::{Store, StoreActionBasic};
use crate::routes::company::company_route;
use crate::routes::job::job_route;
use crate::routes::resume::resume_route;

mod models;
mod routes;
mod controllers;
mod config;
mod middleware;

#[tokio::main]
async fn main() {
    let config = config::config::Config::new().expect("Config env not set");

    let store = setup_store(&config).await;

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


    warp::serve(routes).run(([127, 0, 0, 1], config.port)).await;
}

pub async fn setup_store(config: &Config) -> Store {
    let store: Store = <Store as StoreActionBasic>::new(&format!(
        "postgres://{}:{}@{}:{}/{}",
        config.db_user,
        config.db_password,
        config.db_host,
        config.db_port,
        config.db_name
    )).await;

    store
}

pub struct OneshotHandler {
    pub sender: Sender<i32>
}
pub async fn oneshot(store: Store, address_listen: String) -> OneshotHandler
{
    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[
            Method::PUT,
            Method::DELETE,
            Method::GET,
            Method::POST,
        ]);

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

    let (tx, rx) = oneshot::channel::<i32>();
    let socket: std::net::SocketAddr = address_listen
        .parse()
        .expect("Not a valid address");

    let (_, server) = warp::serve(routes).bind_with_graceful_shutdown(socket, async {
        rx.await.ok();
    });
    tokio::task::spawn(server);
    OneshotHandler{sender:tx}
}
