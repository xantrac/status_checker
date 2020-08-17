use actix::prelude::*;
use actix_files::NamedFile;
use actix_web::client::Client;
use actix_web::middleware::Logger;
use actix_web::{web, Error, HttpRequest, HttpResponse, Responder, Result};
use actix_web_actors::ws;
use env_logger::Env;
use std::{
    thread,
    time::{Duration, Instant},
};

const STATUS_CHECK_INTERVAL: Duration = Duration::from_secs(5);
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

async fn index() -> Result<NamedFile> {
    Ok(NamedFile::open("./static/index.html")?)
}

/// do websocket handshake and start `WebSocket` actor
async fn ws_index(r: HttpRequest, stream: web::Payload) -> HttpResponse {
    println!("{:?}", r);
    let (address, res) = ws::start_with_addr(MyWebSocket::new(), &r, stream).unwrap();

    res
}

struct MyWebSocket {
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    hb: Instant,
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start.
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
        self.status_check(ctx)
    }
}

impl Handler<StatusMessage> for MyWebSocket {
    type Result = Result<bool, std::io::Error>;

    fn handle(&mut self, msg: StatusMessage, ctx: &mut Self::Context) -> Self::Result {
        println!("MESSAGE received ");

        Ok(true)
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        // process websocket messages
        println!("WS: {:?}", msg);
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}

impl MyWebSocket {
    fn new() -> Self {
        Self { hb: Instant::now() }
    }

    /// helper method that sends ping to client every second.
    ///
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");

                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            ctx.ping(b"");
        });
    }

    fn status_check(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(STATUS_CHECK_INTERVAL, |act, ctx| {
            ctx.text("BANANA");
        });
    }
}

async fn get_status() -> impl Responder {
    let mut client = Client::default();

    // Create request builder and send request
    let response = client
        .get("https://kctbh9vrtdwd.statuspage.io/api/v2/status.json")
        .header("User-Agent", "actix-web/3.0")
        .send() // <- Send request
        .await; // <- Wait for response
    let body = response.unwrap().body().await;
    println!("Response: {:?}", body);
    body
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use actix_files::Files;
    use actix_web::{web, App, HttpServer};
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(|| {
        App::new()
            .route("/github", web::get().to(get_status))
            .service(web::resource("/ws/").route(web::get().to(ws_index)))
            .route("/", web::get().to(index))
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(Files::new("/assets", "./static"))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
