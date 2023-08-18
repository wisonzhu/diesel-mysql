#[macro_use]
extern crate diesel;

use actix_web::{App, HttpServer, middleware};
use env_logger::Env;

pub mod route;
pub mod config;
pub mod service;
pub mod model;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let pool = config::database::establish_connection();
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .data(pool.clone())
            .service(route::hello_route::hello)
            .service(route::hello_route::get_userinfo)
            .service(route::hello_route::list_users)
            .service(route::hello_route::create_user)
            .service(route::hello_route::delete_user)
            .service(route::hello_route::jsondemo)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}