#![warn(clippy::all)]
use std::sync::Arc;
use rand::distributions::uniform::SampleBorrow;
use tracing_subscriber::fmt::format::FmtSpan;
use warp::{http::Method, Filter};
use tokio::sync::{oneshot, oneshot::Sender};
use tracing::{info, instrument};
use handle_errors::{Error, return_error};
use routes::user::user_route;
use crate::config::config::Config;
use crate::models::store_db::DatabaseStore;
use crate::models::store_in_memory::InMemoryStore;
use crate::models::store_trait::StoreMethods;
use crate::routes::company::company_route;
use crate::routes::job::job_route;
use crate::routes::resume::resume_route;
use crate::service::telemetry::init_telemetry;

mod models;
mod routes;
mod controllers;
mod config;
mod middleware;
mod service;
mod tests;

#[tokio::main]
#[instrument]
async fn main() {
    let config = Config::new().expect("Config env not set");
    let log_filter = format!(
        "handle_errors={},rust-api-service={},warp={}",
        config.log_level, config.log_level, config.log_level
    );
    init_telemetry(
        config.service_name.as_str(),
        config.server.host.as_str(),
        &config.server.jaeger_port,
        log_filter.as_str()
    );

    let store = build_store(&config).await;
    let routes = build_routes(store).await;

    let address_listen = format!("{}:{}", config.server.host, config.server.port);
    let socket: std::net::SocketAddr = address_listen
        .parse()
        .expect("Not a valid address");
    warp::serve(routes).run(socket).await;
}

pub async fn build_store(config: &Config) -> Arc<dyn StoreMethods + Send + Sync> {
    let url = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.postgres.db_user,
        config.postgres.db_password,
        config.postgres.db_host,
        config.postgres.db_port,
        config.postgres.db_name
    );

    let store: Arc<dyn StoreMethods + Send + Sync> = if config.database.clone().unwrap() == "in-memory".to_string() {
        info!("Using in-memory database");
        Arc::new(InMemoryStore::new())
    } else if config.database.clone().unwrap() == "postgres".to_string(){
        info!("Using postgres database");
        // set up database
        let pool = DatabaseStore::new(&url).await;
        sqlx::migrate!()
            .run(&pool.clone().connection)
            .await
            .map_err(Error::MigrationError)
            .unwrap();
        Arc::new(pool)
    } else {
        info!("Using in-memory database");
        Arc::new(InMemoryStore::new())
    };

    store
}

pub async fn build_routes(store: Arc<dyn StoreMethods + Send + Sync>)
    -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
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
    routes
}

pub async fn init_mock_server(address_listen: String, store: Arc<dyn StoreMethods + Send + Sync>) -> Sender<i32>
{
    let routes = build_routes(store).await;
    let (tx, rx) = oneshot::channel::<i32>();
    let socket: std::net::SocketAddr = address_listen
        .parse()
        .expect("Not a valid address");

    let (_, server) = warp::serve(routes).bind_with_graceful_shutdown(socket, async {
        rx.await.ok();
        info!("Warp server shut down");
    });
    tokio::task::spawn(server);
    tx
}
