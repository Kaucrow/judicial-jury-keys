use jjk_tx::{
    prelude::*,
    settings::get_settings,
    routes::upload,
    telemetry,
};

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    clearscreen::clear()?;

    let settings = get_settings()?;

    // Init the tracing subscriber
    let (subscriber, _guard) = telemetry::get_subscriber(&settings).await?;
    telemetry::init_subscriber(subscriber);

    let host = settings.tx.host;
    let port = settings.tx.port;

    info!("JJK-TX Server listening on {}:{}", host, port);
    info!("Taking uploads on {}:{}/{}", host, port, settings.tx.upload_endp);

    HttpServer::new(move || {
        App::new()
            .route(
                &format!("/{}", settings.tx.upload_endp),
                web::post().to(upload)
            )
    })
    .bind((host, port))?
    .run()
    .await?;

    Ok(())
}