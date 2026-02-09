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

    info!("Server listening on {}:{}", settings.tx.host, settings.tx.port);
    info!("Taking uploads on {}:{}/{}", settings.tx.host, settings.tx.port, settings.tx.upload_endp);

    HttpServer::new(move || {
        App::new()
            .route(
                &format!("/{}", settings.tx.upload_endp),
                web::post().to(upload)
            )
    })
    .bind((settings.tx.host, settings.tx.port))?
    .run()
    .await?;

    Ok(())
}