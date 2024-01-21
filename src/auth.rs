use actix_web::dev::ServiceRequest;
use actix_web::Error;
use actix_web_httpauth::extractors::basic::BasicAuth;
use std::env;

pub async fn env_validate(
    req: ServiceRequest,
    credentials: BasicAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let username = env::var("BASICAUTH_USER").unwrap_or("".to_string());
    let password = env::var("BASICAUTH_PASS").unwrap_or("".to_string());

    if credentials.user_id() != username || credentials.password() != Some(&password) {
        return Err((
            Error::from(actix_web::error::ErrorUnauthorized("Invalid credentials")),
            req,
        ));
    }

    Ok(req)
}
