use actix::prelude::*;
use actix_files::NamedFile;
use actix_web::{web, Error, HttpRequest, HttpResponse, Result};
use actix_web_actors::ws;

pub async fn index() -> Result<NamedFile> {
    Ok(NamedFile::open("./static/index.html")?)
}

pub async fn ws_index(
    r: HttpRequest,
    stream: web::Payload,
    data: web::Data<Addr<super::MonitorActor>>,
) -> Result<HttpResponse, Error> {
    let (addr, res) = ws::start_with_addr(super::websocket_actor::Websocket::new(), &r, stream)?;
    data.get_ref()
        .do_send(super::monitor_actor::WsRegistration { address: addr });

    Ok(res)
}

pub async fn get_status() -> String {
    super::clients::github_status().await
}
