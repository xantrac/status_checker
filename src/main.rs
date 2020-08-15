use actix_files::NamedFile;
use actix_web::{HttpRequest, Result};
use actix_web::middleware::Logger;
use env_logger::Env;

async fn index(req: HttpRequest) -> Result<NamedFile> {
    Ok(NamedFile::open("./static/index.html")?)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
