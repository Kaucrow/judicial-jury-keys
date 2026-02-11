use crate::prelude::*;
use crate::storage::Database;
use crate::encryption::Decrypter;
use crate::domain::{RxKeyResponse, RxPayload, PdfData, CaseSummary};
use std::path::PathBuf;
use tokio::fs;

pub async fn get_public_key(db: web::Data<Database>) -> impl Responder {
    info!("Generating new key pair for upcoming transmission...");
    
    let pdf_id = Uuid::new_v4().to_string();
    
    match Decrypter::generate_keys() {
        Ok((priv_key, pub_key_pem)) => {
            if let Err(e) = db.insert_keys(pdf_id.clone(), priv_key, pub_key_pem.clone()).await {
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

    let priv_key = match db.get_private_key(pdf_id).await {
        Ok(k) => k,
        Err(_) => {
            error!("PDF ID {} not found", pdf_id);
            return HttpResponse::NotFound().body("PDF ID not found");
        }
    };

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
    
    let pdf_data = match serde_json::from_slice::<PdfData>(&plaintext_bytes) {
        Ok(data) => {
            debug!("Decrypted Data - Title: '{}', Author: '{}'", data.title, data.author);
            data
        }
        Err(e) => {
            error!("Failed to deserialize PDF Data: {}", e);
            return HttpResponse::BadRequest().body("Invalid PDF payload");
        }
    };

    let out_dir = PathBuf::from("./out");
    if let Err(e) = fs::create_dir_all(&out_dir).await {
        error!("Failed to create output directory: {}", e);
        return HttpResponse::InternalServerError().body("Storage Error");
    }

    let file_path = out_dir.join(format!("{}.pdf", pdf_id));
    if let Err(e) = fs::write(&file_path, &pdf_data.file).await {
        error!("Failed to write PDF file: {}", e);
        return HttpResponse::InternalServerError().body("Storage Error");
    }

    if let Err(e) = db.update_file_path(pdf_id, &file_path.to_string_lossy()).await {
        error!("Failed to update DB record: {}", e);
        return HttpResponse::InternalServerError().body("DB Update Error");
    }

    info!("Transmission successful for PDF ID: {}", pdf_id);
    HttpResponse::Ok().body("Transmission received and verified successfully")
}

pub async fn list_cases(db: web::Data<Database>) -> impl Responder {
    let cases: Vec<CaseSummary> = match db.list_cases().await {
        Ok(items) => items,
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };

    HttpResponse::Ok().json(cases)
}

pub async fn download_case(
    db: web::Data<Database>,
    path: web::Path<String>,
) -> impl Responder {
    let case_code = path.into_inner();

    let file_path = match db.get_case_file_path(&case_code).await {
        Ok(p) => p,
        Err(_) => return HttpResponse::NotFound().body("Case not found"),
    };

    let bytes = match fs::read(&file_path).await {
        Ok(data) => data,
        Err(_) => return HttpResponse::NotFound().body("PDF not found"),
    };

    HttpResponse::Ok()
        .content_type("application/pdf")
        .append_header(("Content-Disposition", format!("attachment; filename=\"{}.pdf\"", case_code)))
        .body(bytes)
}
