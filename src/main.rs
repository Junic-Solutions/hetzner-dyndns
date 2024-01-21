mod auth;
mod hetzner;

use crate::hetzner::{get_record, update_record};
use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use actix_web_httpauth::middleware::HttpAuthentication;
use env_logger::Env;
use serde::Deserialize;

#[derive(Debug)]
enum Error {
    Http(awc::error::SendRequestError),
    Json(awc::error::JsonPayloadError),
    SerdeJson(serde_json::Error),
}

impl From<awc::error::SendRequestError> for Error {
    fn from(err: awc::error::SendRequestError) -> Self {
        Error::Http(err)
    }
}

impl From<awc::error::JsonPayloadError> for Error {
    fn from(err: awc::error::JsonPayloadError) -> Self {
        Error::Json(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::SerdeJson(err)
    }
}

#[derive(Deserialize)]
struct UpdateQueryParams {
    record: String,
    ip: String,
}

#[get("/")]
async fn update(params: web::Query<UpdateQueryParams>) -> impl Responder {
    let record_id = &params.record;
    let ip = &params.ip;

    let mut record = match get_record(record_id).await {
        Ok(record) => record,
        Err(err) => {
            log::error!("Error getting record: {:?}", err);
            return HttpResponse::InternalServerError();
        }
    };

    record.set_value(ip);

    match update_record(&record).await {
        Ok(_) => HttpResponse::Ok(),
        Err(err) => {
            log::error!("Error updating record: {:?}", err);
            return HttpResponse::InternalServerError();
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        let auth = HttpAuthentication::basic(auth::env_validate);
        App::new()
            .wrap(Logger::default())
            .wrap(auth)
            .service(update)
    })
    .workers(1)
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
