use crate::prelude::*;
use crate::{
    transmission::encryption::perform_hybrid_encryption,
    settings::get_settings,
};

pub async fn upload(
    mut payload: Multipart,
) -> Result<HttpResponse, actix_web::Error> {
    // Iterate over the multipart stream (fields)
    while let Ok(Some(mut field)) = payload.try_next().await {
        match field.content_type() {
            Some(ct) if ct.subtype() == "pdf" => {
                // It is a PDF, proceed to processing below
            },
            _ => {
                // Either it's None, or the subtype is not "pdf"
                return Ok(HttpResponse::BadRequest().body("File must be a .pdf"));
            }
        }

        // Read the file content into memory
        let mut file_bytes = Vec::new();
        while let Some(chunk) = field.next().await {
            let data = chunk.map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
            file_bytes.extend_from_slice(&data);
        }

        // Encrypt the bytes
        // Return immediately after processing the first file found
        match perform_hybrid_encryption(&file_bytes).await {
            Ok(pkg) => {
                let client = reqwest::Client::new();

                let settings = get_settings()
                    .map_err(actix_web::error::ErrorInternalServerError)?;

                // Send the encrypted package to RX
                debug!("Sending encrypted package to RX...");

                let rx_response = client.post(
                    format!(
                        "http://{}:{}/{}", settings.rx.host, settings.rx.port, settings.rx.rcv_endp)
                    )
                    .json(&pkg)
                    .send()
                    .await
                    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
                
                let raw_status = rx_response.status().as_u16();
                let status = actix_web::http::StatusCode::from_u16(raw_status)
                    .unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);

                let body = rx_response.text().await.unwrap_or_else(|_| "Could not read rx response".to_string());

                debug!("Encrypted package sent");

                return Ok(HttpResponse::build(status).body(body))
            },
            Err(e) => return Ok(HttpResponse::InternalServerError().body(format!("{:#}", e))),
        }
    }

    Ok(HttpResponse::BadRequest().body("No file found in request"))
}