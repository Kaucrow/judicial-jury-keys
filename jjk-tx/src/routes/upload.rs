use crate::prelude::*;
use crate::{
    encryption::Encrypter,
    transmission::Transmitter,
    pdf::PdfParser,
};

pub async fn upload(
    mut payload: Multipart,
) -> Result<HttpResponse, actix_web::Error> {
    // Iterate over the multipart stream (fields)
    while let Ok(Some(field)) = payload.try_next().await {
        match field.content_type() {
            Some(ct) if ct.subtype() == "pdf" => {
                // It is a PDF, proceed to processing below
            },
            _ => {
                // Either it's None, or the subtype is not "pdf"
                return Ok(HttpResponse::BadRequest().body("File must be a .pdf"));
            }
        }

        // Parse the PDF data
        let msg = PdfParser::parse(field).await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

        // Serialize the PDF data
        let msg_bytes = serde_json::to_vec(&msg)
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

        // Encrypt the bytes
        // Return immediately after processing the first file found
        match Encrypter::perform_hybrid_encryption(&msg_bytes).await {
            Ok(pkg) => {
                // Send the encrypted package to RX
                let rx_response = Transmitter::send_encrypted_pkg(&pkg).await
                    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

                return Ok(rx_response)
            },
            Err(e) => return Ok(HttpResponse::InternalServerError().body(format!("{:#}", e))),
        }
    }

    Ok(HttpResponse::BadRequest().body("No file found in request"))
}