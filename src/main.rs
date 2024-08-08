use tracing_subscriber::fmt::format::FmtSpan;
use warp::{http::Method, Filter};
use warp::multipart::Part;
use handle_errors::return_error;
use routes::user::user_route;
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
    let configEnv = config::configEnv::ConfigEnv::new().expect("Config env not set");
    let log_filter = format!(
        "handle_errors={},rust_web_dev={},warp={}",
        configEnv.log_level, configEnv.log_level, configEnv.log_level
    );
    // let db = config::connectDB::init_db().await.expect("Can't init DB");
    // let db = Arc::new(db);
    let db_url = &format!(
        "postgres://{}:{}@{}:{}/{}",
        configEnv.db_user,
        configEnv.db_password,
        configEnv.db_host,
        configEnv.db_port,
        configEnv.db_name
    );
    let store: Store = <Store as StoreActionBasic>::new(&db_url).await;
    // let store = Arc::new(store);


    tracing_subscriber::fmt()
        // Use the filter we built above to determine which traces to record.
        .with_env_filter(log_filter)
        // Record an event when each span closes. This can be used to time our
        // routes' durations!
        .with_span_events(FmtSpan::CLOSE)
        .init();

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

    warp::serve(routes).run(([127, 0, 0, 1], configEnv.port)).await;
}
