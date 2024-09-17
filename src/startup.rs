use crate::routes::*;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    // wrap the connection in a smart pointer
    let db_pool = web::Data::new(db_pool);
    // capture 'connection' from the surrounding environment
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            // Get a pointer copy and aattach it to the application state.
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
