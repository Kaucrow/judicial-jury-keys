use actix_web::{
    error::ErrorBadRequest,
    http::header,
    middleware,
    web,
    App, Error, HttpResponse, HttpServer,
};
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Key, Nonce,
};
use base64::{engine::general_purpose::STANDARD as B64, Engine as _};
use rsa::{
    oaep::Oaep,
    pkcs8::DecodePrivateKey,
    RsaPrivateKey,
};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

#[derive(Serialize)]
struct HealthResponse {
    ok: bool,
}

#[derive(Deserialize)]
struct SymmetricDecryptRequestJson {
    encrypted_pdf_b64: String,
    aes_key_b64: String,
    nonce_b64: String,
}

#[derive(Deserialize)]
struct AsymmetricDecryptRequestJson {
    encrypted_pdf_b64: String,
    encrypted_aes_key_b64: String,
    nonce_b64: String,
    private_key_pem: String,
}

async fn health() -> HttpResponse {
    HttpResponse::Ok().json(HealthResponse { ok: true })
}

fn aes_decrypt(ciphertext: &[u8], key_bytes: &[u8; 32], nonce_bytes: &[u8; 12]) -> Result<Vec<u8>, Error> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key_bytes));
    cipher
        .decrypt(Nonce::from_slice(nonce_bytes), ciphertext)
        .map_err(|_| ErrorBadRequest("AES-GCM decrypt failed (key/nonce or data invalid)"))
}


fn rsa_oaep_decrypt(private_key_pem: &str, ciphertext: &[u8]) -> Result<Vec<u8>, Error> {
    let private = RsaPrivateKey::from_pkcs8_pem(private_key_pem)
        .map_err(|_| ErrorBadRequest("Invalid private_key_pem"))?;
    private
        .decrypt(Oaep::new::<Sha256>(), ciphertext)
        .map_err(|_| ErrorBadRequest("RSA OAEP decrypt failed"))
}

async fn decrypt_symmetric_json(req: web::Json<SymmetricDecryptRequestJson>) -> Result<HttpResponse, Error> {
    let encrypted_pdf = B64
        .decode(req.encrypted_pdf_b64.as_bytes())
        .map_err(|_| ErrorBadRequest("encrypted_pdf_b64 is not valid base64"))?;
    let key = B64
        .decode(req.aes_key_b64.as_bytes())
        .map_err(|_| ErrorBadRequest("aes_key_b64 is not valid base64"))?;
    let nonce = B64
        .decode(req.nonce_b64.as_bytes())
        .map_err(|_| ErrorBadRequest("nonce_b64 is not valid base64"))?;

    let key: [u8; 32] = key
        .try_into()
        .map_err(|_| ErrorBadRequest("aes_key_b64 must decode to 32 bytes"))?;
    let nonce: [u8; 12] = nonce
        .try_into()
        .map_err(|_| ErrorBadRequest("nonce_b64 must decode to 12 bytes"))?;

    let pdf = aes_decrypt(&encrypted_pdf, &key, &nonce)?;
    Ok(HttpResponse::Ok()
        .insert_header((header::CONTENT_TYPE, "application/pdf"))
        .body(pdf))
}

async fn decrypt_asymmetric_json(req: web::Json<AsymmetricDecryptRequestJson>) -> Result<HttpResponse, Error> {
    let encrypted_pdf = B64
        .decode(req.encrypted_pdf_b64.as_bytes())
        .map_err(|_| ErrorBadRequest("encrypted_pdf_b64 is not valid base64"))?;
    let encrypted_key = B64
        .decode(req.encrypted_aes_key_b64.as_bytes())
        .map_err(|_| ErrorBadRequest("encrypted_aes_key_b64 is not valid base64"))?;
    let nonce = B64
        .decode(req.nonce_b64.as_bytes())
        .map_err(|_| ErrorBadRequest("nonce_b64 is not valid base64"))?;
    let nonce: [u8; 12] = nonce
        .try_into()
        .map_err(|_| ErrorBadRequest("nonce_b64 must decode to 12 bytes"))?;

    let key_bytes = rsa_oaep_decrypt(&req.private_key_pem, &encrypted_key)?;
    let key: [u8; 32] = key_bytes
        .try_into()
        .map_err(|_| ErrorBadRequest("RSA-decrypted AES key must be 32 bytes"))?;

    let pdf = aes_decrypt(&encrypted_pdf, &key, &nonce)?;
    Ok(HttpResponse::Ok()
        .insert_header((header::CONTENT_TYPE, "application/pdf"))
        .body(pdf))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let bind_addr = std::env::var("JJK_RX_BIND").unwrap_or_else(|_| "127.0.0.1:8081".to_string());

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/health", web::get().to(health))
            .route("/decrypt/symmetric", web::post().to(decrypt_symmetric_json))
            .route("/decrypt/asymmetric", web::post().to(decrypt_asymmetric_json))
    })
    .bind(bind_addr)?
    .run()
    .await
}
