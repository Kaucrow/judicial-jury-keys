use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize, sqlx::FromRow)]
pub struct Usuario {
    pub id: i32,
    pub nombre: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, sqlx::FromRow)]
pub struct Pdf {
    pub id: i32,
    pub public_key: String,
    pub private_key: String,
    pub file_path: String,
    pub record_num: String,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub description: Option<String>,
}
