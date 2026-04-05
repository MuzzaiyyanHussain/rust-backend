use actix_web::{web, App, HttpServer};
use dotenv::dotenv;

mod routes;
mod handlers;
mod db;
mod auth;

use routes::init_routes;
use db::connect_db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); 

    let pool = connect_db().await; 

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) 
            .configure(init_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}