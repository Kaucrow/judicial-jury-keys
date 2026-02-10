use tracing_subscriber::fmt::format::FmtSpan;

mod prelude;
mod domain;
mod encryption;
mod storage;
mod routes;
mod db; 

use prelude::*;
use storage::Database;
use routes::handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _ = clearscreen::clear();

    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter("jjk_rx=debug,actix_server=off,actix_web=off")
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env or env vars");
    let db = Database::connect(&database_url).await.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    let db_data = web::Data::new(db);

    info!("Server listening on 0.0.0.0:8081");
    info!("Endpoints: /public_key (GET), /receive (POST)");

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .route("/public_key", web::get().to(handlers::get_public_key))
            .route("/receive", web::post().to(handlers::receive_package))
            .route("/cases", web::get().to(handlers::list_cases))
            .route("/download/{caseCode}", web::get().to(handlers::download_case))
    })
    .bind(("0.0.0.0", 8081))?
    .run()
    .await
}
