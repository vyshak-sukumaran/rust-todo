use actix_web::{App, HttpServer};
use env_logger;
use log::{info, LevelFilter};

pub mod controllers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::builder().filter_level(LevelFilter::Info).init();

    let host: &str = "127.0.0.1";
    let port: u16 = 8080;

    info!("The server started at: http://{}:{}", host, port);

    HttpServer::new(|| App::new().configure(controllers::config))
        .bind((host.to_string(), port))?
        .run()
        .await
}
