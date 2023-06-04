use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;

use crate::routes::{health_check, subscribe};

pub async fn run(listener: TcpListener, connection: PgPool) -> Result<Server, std::io::Error> {
    let conn = web::Data::new(connection);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(conn.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
