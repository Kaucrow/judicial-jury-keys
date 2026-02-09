use tracing_subscriber::fmt::format::FmtSpan;

mod prelude;
mod domain;
mod encryption;
mod storage;
mod routes;

use prelude::*;
use storage::Database;
use routes::handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Init logging
    tracing_subscriber::fmt()
        .with_env_filter("info,jjk_rx=debug")
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let db = Database::new();
    let db_data = web::Data::new(db);

    tracing::info!("JJK-RX Server starting on 127.0.0.1:8081");

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .route("/public_key", web::get().to(handlers::get_public_key))
            .route("/receive", web::post().to(handlers::receive_package))
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
