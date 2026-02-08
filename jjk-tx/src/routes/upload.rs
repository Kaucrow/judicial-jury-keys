use crate::prelude::*;
use crate::{
    encryption::Encrypter,
    transmission::Transmitter,
    pdf::PdfParser,
};

pub async fn upload(
    mut payload: Multipart,
) -> HttpResponse {
    let result: anyhow::Result<HttpResponse> = async move {
        // Iterate over the multipart stream (fields)
        while let Ok(Some(field)) = payload.try_next().await {
            match field.content_type() {
                Some(ct) if ct.subtype() == "pdf" => {},
                _ => return Ok(HttpResponse::BadRequest().body("File must be a .pdf")),
            }

            // Parse the PDF data
            let msg = PdfParser::parse(field).await?; 

            // Serialize the PDF data
            let msg_bytes = serde_json::to_vec(&msg)?;

            // Encrypt the bytes
            // Returns immediately after processing the first file found
            match Encrypter::perform_hybrid_encryption(&msg_bytes).await {
                Ok((pdf_id, pkg)) => {
                    let rx_response = Transmitter::send_encrypted_pkg(pdf_id, pkg).await?;
                    return Ok(rx_response);
                },
                Err(e) => {
                    debug!("Encryption failed: {:#}", e);
                    return Err(e);
                }
            }
        }

        Ok(HttpResponse::BadRequest().body("No file found in request"))
    }.await;

    match result {
        Ok(response) => response,
        Err(e) => {
            error!("Upload failed: {}", e);
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}