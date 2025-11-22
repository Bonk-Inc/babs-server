use std::env;

use axum::{extract::Request, http::HeaderMap, middleware::Next, response::Response};
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use log::{error, info};
use serde::{Deserialize, Serialize};

use crate::{
    response::{ErrorResponse, ResponseBody},
    service::oauth2_service,
};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: String,
    sub: String,
    exp: usize,
}

/// This function validates if the given request contains a valid OAuth2 access token using the given JWKS token from
/// the authorization server.
///
/// # Errors
/// - if no Authorization header is present.
/// - if the JWKS tokens could not be read.
/// - if the JWKS tokens could not be decoded.
/// - If the given access token is invalid.
pub async fn verify_token(
    headers: HeaderMap,
    req: Request,
    next: Next,
) -> Result<Response, ErrorResponse> {
    let audience = env::var("OAUTH_CLIENT_ID").expect("OAUTH_CLIENT_ID must be set");
    let jwk_token = oauth2_service::get_jwk_tokens();
    let auth_token = headers.get("Authorization");

    if auth_token.is_none() || jwk_token.is_err() {
        return Err(ResponseBody::unauthorized_error("Invalid token"));
    }

    let token = auth_token.unwrap().to_str().unwrap().replace("Bearer ", "");
    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&[audience]);

    let decoding_key = match jwk_token.unwrap() {
        Some(token) => match DecodingKey::from_jwk(&token) {
            Ok(key) => key,
            Err(_) => {
                error!("Could not decode the JWK");

                return Err(ResponseBody::unauthorized_error(
                    "Error during authenticating",
                ));
            }
        },
        None => {
            error!("Could not get a JWK");

            return Err(ResponseBody::unauthorized_error(
                "Error during authenticating",
            ));
        }
    };

    match jsonwebtoken::decode::<Claims>(&token, &decoding_key, &validation) {
        Ok(_) => info!("User authenticated"),
        Err(err) => {
            info!(
                "User authentication failed, invalid token. Reason '{:?}'",
                err.kind()
            );

            return Err(ResponseBody::unauthorized_error("Invalid token"));
        }
    }

    Ok(next.run(req).await)
}
