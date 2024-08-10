use chrono::Utc;
use warp:: {
    filters::header::headers_cloned,
    http::header::{HeaderMap, HeaderValue, AUTHORIZATION},
    Filter
};
use handle_errors::Error;
use crate::middleware::jwt::{Jwt, Claims, JwtActions};

const BEARER: &str = "Bearer";

pub fn auth(role: i32)
    -> impl Filter<Extract = (Claims,), Error = warp::Rejection> + Clone
{
    headers_cloned()
        .map(move |headers: HeaderMap<HeaderValue>| (role.clone(), headers))
        .and_then(authorize)
}

async fn authorize ((role, headers): (i32, HeaderMap<HeaderValue>))
    -> Result<Claims, warp::Rejection>
{
    match jwt_from_header(&headers) {
        Ok(token) => {
            match Jwt::verify_access_token(&token) {
                Ok(claims) => {
                    let current_date_time = Utc::now();
                    if claims.is_delete == true {
                        return Err(warp::reject())
                    }
                    if claims.exp < current_date_time.timestamp() as usize {
                        return Err(warp::reject())
                    }
                    if claims.role.0 != role {
                        return Err(warp::reject())
                    };
                    Ok(claims)
                },
                Err(e) => Err(warp::reject::custom(e)),
            }
        },
        Err(e) => Err(warp::reject::custom(e)),
    }
}

fn jwt_from_header(headers: &HeaderMap<HeaderValue>)
    -> Result<String, Error>
{
    let header = match headers.get(AUTHORIZATION) {
        Some(v) => v,
        None => return Err(Error::Unauthorized),
    };

    let auth_header = match std::str::from_utf8(header.as_bytes()) {
        Ok(v) => v,
        Err(e) => {
            tracing::event!(tracing::Level::ERROR, "{:?}", e);
            return Err(Error::Unauthorized)
        }
    };

    if !auth_header.starts_with(BEARER) {
        return Err(Error::Unauthorized)
    }
    Ok(auth_header.trim_start_matches(BEARER).to_owned())
}
