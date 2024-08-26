use crate::service::handle_errors::Error;
use crate::service::jwt::{Claims, Jwt, JwtActions};
use chrono::Utc;
use tracing::{event, Level};
use warp::{
    filters::header::headers_cloned,
    http::header::{HeaderMap, HeaderValue, AUTHORIZATION},
    Filter,
};

const BEARER: &str = "Bearer";

// Authentication
pub fn auth(role_id: i32) -> impl Filter<Extract = (Claims,), Error = warp::Rejection> + Clone {
    headers_cloned()
        .map(move |headers: HeaderMap<HeaderValue>| (role_id, headers))
        .and_then(authorize)
}

// Handle authentication
//
// # Arguments
// * `role_id` - A id of the role.
// * `headers` - Headers values of request.
//
// # Return
// A claim data decoded from access token.
//```

async fn authorize(
    (role_id, headers): (i32, HeaderMap<HeaderValue>),
) -> Result<Claims, warp::Rejection> {
    // Get access token from headers
    match jwt_from_header(&headers) {
        Ok(token) => match Jwt::verify_access_token(&token) {
            Ok(claims) => {
                let current_date_time = Utc::now();
                if claims.is_delete {
                    return Err(warp::reject());
                }
                if claims.exp < current_date_time.timestamp() as usize {
                    return Err(warp::reject());
                }
                if claims.role_id.0 != role_id {
                    return Err(warp::reject());
                };
                Ok(claims)
            }
            Err(e) => Err(warp::reject::custom(e)),
        },
        Err(e) => Err(warp::reject::custom(e)),
    }
}

// Handle takes jwt token from header
//
// # Arguments
// * `headers` - Headers values of request.
//
// # Return
// A access token.
//```
fn jwt_from_header(headers: &HeaderMap<HeaderValue>) -> Result<String, Error> {
    // Get header value from header with key value is AUTHORIZATION
    let header = match headers.get(AUTHORIZATION) {
        Some(v) => v,
        None => return Err(Error::Unauthorized),
    };

    // Convert header value as &[u8] to &str
    let auth_header = match std::str::from_utf8(header.as_bytes()) {
        Ok(v) => v,
        Err(e) => {
            event!(
                Level::ERROR,
                "Convert header value as &[u8] to &str has error: {:?}",
                e
            );
            return Err(Error::Utf8(e));
        }
    };

    // Check auth header string started with string 'bearer'
    if !auth_header.starts_with(BEARER) {
        event!(Level::ERROR, "Missing bearer auth type");
        return Err(Error::MissingBearerAuthType);
    }
    Ok(auth_header.trim_start_matches(BEARER).to_owned())
}
