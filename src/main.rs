#[macro_use]
extern crate diesel;

use actix_web::{App, HttpServer, middleware};
use env_logger::Env;

pub mod routes;
pub mod config;
pub mod services;
pub mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let pool = config::database::establish_connection();
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .data(pool.clone())
            .service(routes::route::hello)
            .service(routes::route::get_userinfo)
            .service(routes::route::list_users)
            .service(routes::route::create_user)
            .service(routes::route::delete_user)
            .service(routes::route::update_user)
            .service(routes::route::jsondemo)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}