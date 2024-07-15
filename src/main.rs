//Instantiate Files For Discovery
mod models;
mod schema;
mod db;
mod backend;

//Imports
use backend::*;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> { 
    let port = 8080;
    HttpServer::new(move || {
        App::new()
        .service(user_details)
        .service(user_purchases)
        .service(create_user)
        .service(create_product)
        .service(create_order)
        .service(create_order_item)
    })
    .bind(("0.0.0.0", port))?   
    .workers(6)
    .run()
    .await
}
