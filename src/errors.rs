use thiserror::Error;
use tracing::{event, Level};
use warp::{
    filters::{body::BodyDeserializeError, cors::CorsForbidden},
    http::StatusCode,
    reject::Reject,
    Rejection, Reply,
};

#[derive(Error, Debug)]
pub enum Error {
    //Error of database
    #[error("Database query error, invalid data")]
    DatabaseQuery(#[from] sqlx::Error),

    //Error of In Memory
    #[error("Not found data")]
    NotFound,

    //Error of token
    #[error("Can't decrypt token error")]
    CannotDecryptToken,
    #[error("Can't encrypt token error")]
    CannotEncryptToken,

    //Error of hash and verify password
    #[error("Can't verify password")]
    ArgonLibrary(#[from] argon2::Error),
    #[error("Un authenticated")]
    WrongPassword,

    //Error of authorized user and authenticated user
    #[error("No permission to change the underlying resource")]
    Unauthorized,
    #[error("UnAuthenticated")]
    Unauthenticated,
    #[error("Utf8 error: {0}")]
    Utf8(#[from] std::str::Utf8Error),
    #[error("Missing bearer auth type")]
    MissingBearerAuthType,

    #[error("Can't parse parameter: {0}")]
    Parse(#[from] std::num::ParseIntError),
    #[error("Can't migrate data")]
    Migration(#[from] sqlx::migrate::MigrateError),
    #[error("Load configs error: {0}")]
    LoadConfigErr(#[from] serde_yaml::Error),
    #[error("Missing parameter")]
    MissingParameters,
}

impl Reject for Error {}
// search in
//https://www.ibm.com/docs/en/db2-for-zos/13?topic=codes-sqlstate-values-common-error#db2z_sqlstatevalues__classcode02
const DUPLICATE_KEY: u32 = 23505;

pub async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(Error::DatabaseQuery(e)) = r.find() {
        event!(Level::ERROR, "Database query error");
        match e {
            sqlx::Error::Database(err) => {
                if err.code().unwrap().parse::<u32>().unwrap() == DUPLICATE_KEY {
                    Ok(warp::reply::with_status(
                        "User already exists".to_string(),
                        StatusCode::UNPROCESSABLE_ENTITY,
                    ))
                } else {
                    Ok(warp::reply::with_status(
                        "Can't update data".to_string(),
                        StatusCode::UNPROCESSABLE_ENTITY,
                    ))
                }
            }
            _ => Ok(warp::reply::with_status(
                "Not Found".to_string(),
                StatusCode::NOT_FOUND,
            )),
        }
    } else if let Some(Error::NotFound) = r.find() {
        Ok(warp::reply::with_status(
            "Not Found Data".to_string(),
            StatusCode::NOT_FOUND,
        ))
    } else if let Some(Error::WrongPassword) = r.find() {
        Ok(warp::reply::with_status(
            "Wrong E-Mail/Password combination".to_string(),
            StatusCode::UNAUTHORIZED,
        ))
    } else if let Some(Error::Unauthorized) = r.find() {
        event!(Level::ERROR, "Not matching account id");
        Ok(warp::reply::with_status(
            "No permission to change underlying resource".to_string(),
            StatusCode::UNAUTHORIZED,
        ))
    } else if let Some(Error::Unauthenticated) = r.find() {
        event!(Level::ERROR, "Un authenticated");
        Ok(warp::reply::with_status(
            "Un authenticated".to_string(),
            StatusCode::UNAUTHORIZED,
        ))
    } else if let Some(Error::MissingBearerAuthType) = r.find() {
        event!(Level::ERROR, "Un authenticated");
        Ok(warp::reply::with_status(
            "Missing bearer auth type in header".to_string(),
            StatusCode::UNAUTHORIZED,
        ))
    } else if let Some(error) = r.find::<CorsForbidden>() {
        event!(Level::ERROR, "CORS forbidden error: {}", error);
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::FORBIDDEN,
        ))
    } else if let Some(error) = r.find::<BodyDeserializeError>() {
        event!(Level::ERROR, "Can't deserialize request body: {}", error);
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else if let Some(error) = r.find::<Error>() {
        event!(Level::ERROR, "{}", error);
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else {
        event!(Level::WARN, "Requested route was not found");
        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}
