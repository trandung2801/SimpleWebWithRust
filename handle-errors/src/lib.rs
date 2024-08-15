use std::str::Utf8Error;
use warp::{
    filters::{body::BodyDeserializeError, cors::CorsForbidden},
    http::StatusCode,
    reject::Reject,
    Rejection, Reply,
};
use tracing::{event, Level};
use argon2::Error as ArgonError;

#[derive(Debug)]
pub enum Error {
    //Error of database
    DatabaseQueryError(sqlx::Error),

    //Error of token
    CannotDecryptToken,
    CannotEncryptToken,

    //Error of hash and verify password
    ArgonLibraryError(ArgonError),
    WrongPassword,

    //Error of authorized user and authenticated user
    Unauthorized,
    Unauthenticated,
    Utf8Error(Utf8Error),
    MissingBearerAuthType,


    ParseError(std::num::ParseIntError),
    MigrationError(sqlx::migrate::MigrateError),
    LoadConfigErr(serde_yaml::Error),
    MissingParameters,

}


#[derive(Debug, Clone)]
pub struct APILayerError {
    pub status: u16,
    pub message: String,
}

impl std::fmt::Display for APILayerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Status: {}, Message: {}", self.status, self.message)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &*self {
            Error::DatabaseQueryError(_) => write!(f, "Database query error, invalid data"),

            Error::CannotDecryptToken => write!(f, "Can't decrypt token error"),
            Error::CannotEncryptToken => write!(f, "Can't encrypt token error"),

            Error::ArgonLibraryError(_) => write!(f, "Can't verify password"),
            Error::WrongPassword => write!(f, "Wrong password"),

            Error::Unauthorized => write!(f, "No permission to change the underlying resource"),
            Error::Unauthenticated => write!(f, "Un authenticated"),
            Error::Utf8Error(err) => write!(f, "Utf8 error: {}", err),
            Error::MissingBearerAuthType => write!(f, "Missing bearer auth type"),

            Error::ParseError(ref err) => write!(f, "Can't parse parameter: {}", err),
            Error::MissingParameters => write!(f, "Missing parameter"),
            Error::MigrationError(_) => write!(f, "Can't migrate data"),
            Error::LoadConfigErr(err) => write!(f, "Load config error: {}", err),

        }
    }
}

impl Reject for Error {}
impl Reject for APILayerError {}

const DUPLICATE_KEY: u32 = 23505;

pub async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(Error::DatabaseQueryError(e)) = r.find() {
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
            },
            _ => {
                Ok(warp::reply::with_status(
                    "Not Found".to_string(),
                    StatusCode::NOT_FOUND,
                ))
            }
        }
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