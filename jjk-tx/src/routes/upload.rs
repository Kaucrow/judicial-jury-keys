use crate::prelude::*;
use crate::{
    encryption::Encrypter,
    transmission::Transmitter,
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
        match Encrypter::perform_hybrid_encryption(&file_bytes).await {
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