use std::sync::Arc;

use tokio::sync::{oneshot, oneshot::Sender};
use tracing::{info, instrument};
use warp::{http::Method, Filter};

use crate::configs::config::Config;
use crate::errors::{return_error, Error};
use crate::models::store_db::DatabaseStore;
use crate::models::store_in_memory::InMemoryStore;
use crate::models::store_trait::StoreMethods;
use crate::routes::company::company_route;
use crate::routes::job::job_route;
use crate::routes::resume::resume_route;
use crate::routes::user::user_route;
use crate::services::telemetry::init_telemetry;

mod configs;
mod controllers;
pub mod errors;
mod middleware;
mod models;
mod routes;
mod services;
mod tests;
mod utils;

#[tokio::main]
#[instrument]
async fn main() {
    let config = Config::new().expect("Config env not set");
    let log_filter = format!(
        "handle_errors={},rust-api-services={},warp={}",
        config.log_level, config.log_level, config.log_level
    );
    init_telemetry(
        &config.service_name,
        &config.server.host,
        &config.server.jaeger_port,
        log_filter.as_str(),
    );

    let store = build_store(&config).await;
    let routes = build_routes(store).recover(return_error);

    let address_listen = format!("{}:{}", config.server.host, config.server.port);
    let socket: std::net::SocketAddr = address_listen.parse().expect("Not a valid address");
    warp::serve(routes).run(socket).await;
}

pub async fn build_store(config: &Config) -> Arc<dyn StoreMethods + Send + Sync> {
    let url = config.postgres.url.clone();

    let store: Arc<dyn StoreMethods + Send + Sync> =
        if config.database.clone().unwrap() == *"postgres".to_string() {
            info!("Using postgres database");
            // set up database
            let pool = DatabaseStore::new(&url).await;
            sqlx::migrate!()
                .run(&pool.clone().connection)
                .await
                .map_err(Error::Migration)
                .unwrap();
            Arc::new(pool)
        } else {
            info!("Using in-memory database");
            Arc::new(InMemoryStore::new())
        };
    store
}

pub fn build_routes(
    store: Arc<dyn StoreMethods + Send + Sync>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);

    let user_routes = user_route("api", store.clone());
    let company_routes = company_route("api", store.clone());
    let resume_routes = resume_route("api", store.clone());
    let job_routes = job_route("api", store.clone());
    user_routes
        .or(company_routes)
        .or(resume_routes)
        .or(job_routes)
        .with(cors)
        .with(warp::trace::request())
}

pub async fn init_test_server(
    address_listen: String,
    store: Arc<dyn StoreMethods + Send + Sync>,
) -> Sender<i32> {
    let routes = build_routes(store).recover(return_error);
    let (tx, rx) = oneshot::channel::<i32>();
    let socket: std::net::SocketAddr = address_listen.parse().expect("Not a valid address");

    let (_, server) = warp::serve(routes).bind_with_graceful_shutdown(socket, async {
        rx.await.ok();
        info!("Warp server shut down");
    });
    tokio::task::spawn(server);
    tx
}

pub async fn build_store_for_test(
    url: String,
    database: String,
    sample_data_url: String,
) -> Arc<dyn StoreMethods + Send + Sync> {
    let store: Arc<dyn StoreMethods + Send + Sync> = if database == *"postgres".to_string() {
        info!("Using postgres database");
        // set up database
        let pool = DatabaseStore::new(&url).await;

        sqlx::migrate!()
            .run(&pool.clone().connection)
            .await
            .map_err(Error::Migration)
            .unwrap();
        let _ = pool.create_sample_data(&sample_data_url).await;
        Arc::new(pool)
    } else {
        info!("Using in-memory database");
        Arc::new(InMemoryStore::new())
    };
    store
}
