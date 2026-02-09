use jjk_rx::{
    prelude::*,
    settings::get_settings,
    telemetry,
    storage::Database,
    routes::handlers,
};

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    clearscreen::clear()?;

    let settings = get_settings()?;

    // Init the tracing subscriber
    let (subscriber, _guard) = telemetry::get_subscriber(&settings).await?;
    telemetry::init_subscriber(subscriber);

    let db = Database::new();
    let db_data = web::Data::new(db);

    let host = settings.rx.host;
    let port = settings.rx.port;

    info!("JJK-RX Server listening on {}:{}", host, port);
    info!("Serving public keys on {}:{}/{}", host, port, settings.rx.pub_key_endp);
    info!("Taking encrypted PKGs on {}:{}/{}", host, port, settings.rx.rcv_endp);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .route("/public_key", web::get().to(handlers::get_public_key))
            .route("/receive", web::post().to(handlers::receive_package))
    })
    .bind((host, port))?
    .run()
    .await?;

    Ok(())
}