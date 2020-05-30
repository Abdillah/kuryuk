#[macro_use]
extern crate diesel;

mod controllers;
mod response;
mod model;
#[macro_use]
mod schema;
mod query;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{middleware, web, App, HttpServer};

    HttpServer::new(|| {
        App::new()
        .wrap(middleware::Compress::default())
        .route("/transactions", web::get().to(controllers::transactions::read))
        .route("/transactions", web::post().to(controllers::transactions::create))
        .route("/transactions/{id:[0-9]{1,5}}", web::get().to(controllers::transactions::read))
        .route("/transactions/{id:[0-9]{1,5}}", web::patch().to(controllers::transactions::update))
        .route("/transactions/{id:[0-9]{1,5}}", web::delete().to(controllers::transactions::delete))
    })
    .bind("127.0.0.1:9090")?
    .run()
    .await
}
