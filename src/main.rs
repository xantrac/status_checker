use actix::prelude::*;
use actix_web::middleware::Logger;
use env_logger::Env;

mod lib;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use actix_files::Files;
    use actix_web::{web, App, HttpServer};
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    let monitor_actor = lib::MonitorActor::new().start();

    HttpServer::new(move || {
        App::new()
            .data(monitor_actor.clone())
            .route("/github", web::get().to(lib::get_status))
            .service(web::resource("/ws/").route(web::get().to(lib::ws_index)))
            .route("/", web::get().to(lib::index))
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(Files::new("/assets", "./static"))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
