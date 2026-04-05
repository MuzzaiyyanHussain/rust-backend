use actix_web::{App, HttpServer, web};
use dotenv::dotenv;

mod auth;
mod db;
mod handlers;
mod ratelimiter;
mod routes;

use db::connect_db;
use ratelimiter::limiter::RateLimiter;
use routes::init_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let limiter = RateLimiter::new(5, 10);
    dotenv().ok();

    let pool = connect_db().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(limiter.clone()))
            .configure(init_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
