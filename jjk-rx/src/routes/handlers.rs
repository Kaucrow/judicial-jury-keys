use crate::prelude::*;
use crate::storage::Database;
use crate::encryption::Decrypter;
use crate::domain::{RxKeyResponse, RxPayload, PdfData};

pub async fn get_public_key(db: web::Data<Database>) -> impl Responder {
    info!("Generating new key pair for upcoming transmission...");

    // Generate new PDF_ID
    let pdf_id = Uuid::new_v4().to_string();

    // Generate Keys
    match Decrypter::generate_keys() {
        Ok((priv_key, pub_key_pem)) => {
            // Save to DB
            if let Err(e) = db.insert_keys(pdf_id.clone(), priv_key, pub_key_pem.clone()) {
                error!("Failed to save keys to DB: {}", e);
                return HttpResponse::InternalServerError().body("DB Error");
            }

            info!("Keys generated for PDF ID: {}", pdf_id);
            HttpResponse::Ok().json(RxKeyResponse {
                pdf_id,
                pub_key: pub_key_pem,
            })
        },
        Err(e) => {
            error!("Failed to generate keys: {}", e);
            HttpResponse::InternalServerError().body("Key Generation Error")
        }
    }
}

pub async fn receive_package(
    db: web::Data<Database>,
    payload: web::Json<RxPayload>,
) -> impl Responder {
    let pdf_id = &payload.pdf_id;
    let pkg = &payload.pkg;

    info!("Received encrypted package for PDF ID: {}", pdf_id);

    // Get Private Key
    let priv_key = match db.get_private_key(pdf_id) {
        Ok(k) => k,
        Err(_) => {
            error!("PDF ID {} not found", pdf_id);
            return HttpResponse::NotFound().body("PDF ID not found");
        }
    };

    // Decrypt
    let plaintext_bytes = match Decrypter::decrypt_hybrid(
        &priv_key,
        &pkg.encrypted_session_key_b64,
        &pkg.encrypted_data_b64,
        &pkg.nonce_b64
    ) {
        Ok(bytes) => bytes,
        Err(e) => {
            error!("Decryption failed for {}: {}", pdf_id, e);
            return HttpResponse::BadRequest().body(format!("Decryption failed: {}", e));
        }
    };

    // Verify Hash
    match Decrypter::verify_hash(&plaintext_bytes, &pkg.hash_b64) {
        Ok(true) => {
            debug!("Hash verification successful for {}", pdf_id);
        },
        Ok(false) => {
            error!("Hash verification failed for {}", pdf_id);
            return HttpResponse::BadRequest().body("Integrity check failed (Hash mismatch)");
        },
        Err(e) => {
             error!("Hash verification error: {}", e);
             return HttpResponse::BadRequest().body("Hash verification error");
        }
    }

    // Deserialize to PdfData to verify structure
    match serde_json::from_slice::<PdfData>(&plaintext_bytes) {
        Ok(pdf_data) => {
             debug!("Decrypted Data - Title: '{}', Author: '{}'", pdf_data.title, pdf_data.author);
        },
        Err(e) => {
             error!("Failed to deserialize PDF Data: {}", e);
             // Note: We might want to return an error here if the data format is strict
        }
    }

    // Update DB (Simulating "Data Storage" step)
    if let Err(e) = db.update_with_package(pdf_id, pkg.clone()) {
        error!("Failed to update DB record: {}", e);
        return HttpResponse::InternalServerError().body("DB Update Error");
    }

    info!("Transmission successful for PDF ID: {}", pdf_id);
    HttpResponse::Ok().body("Transmission received and verified successfully")
}