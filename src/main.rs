use tracing_subscriber::fmt::format::FmtSpan;
use warp::{http::Method, Filter};
use warp::multipart::Part;
use handle_errors::return_error;
use routes::userRoute::user_route;
use crate::models::store;
use crate::models::store::Store;
use crate::routes::companyRoute::company_route;

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
    let store = store::Store::new(&db_url).await;
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
    let routes = user_routes
        .or(company_routes)
        .with(cors)
        .with(warp::trace::request())
        .recover(return_error);

    warp::serve(routes).run(([127, 0, 0, 1], configEnv.port)).await;
}
