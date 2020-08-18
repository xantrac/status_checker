mod clients;
mod monitor_actor;
mod request_handlers;
mod status_server;
mod websocket_actor;
pub use self::monitor_actor::MonitorActor;
use actix::prelude::*;
use actix_files::NamedFile;
use actix_web::{web, Error, HttpRequest, HttpResponse, Result};

pub async fn index() -> Result<NamedFile> {
    request_handlers::index().await
}

pub async fn ws_index(
    r: HttpRequest,
    stream: web::Payload,
    data: web::Data<Addr<monitor_actor::MonitorActor>>,
) -> Result<HttpResponse, Error> {
    request_handlers::ws_index(r, stream, data).await
}

pub async fn get_status() -> String {
    request_handlers::get_status().await
}
