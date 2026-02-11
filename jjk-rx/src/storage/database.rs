use crate::prelude::*;
use crate::db::db_component::Db;
use crate::domain::CaseSummary;
use rsa::pkcs8::{EncodePrivateKey, DecodePrivateKey};
use sqlx::FromRow;

#[derive(Clone)]
pub struct Database {
    db: Db,
}

impl Database {
    pub async fn connect(database_url: &str) -> Result<Self> {
        let db = Db::connect(database_url, 5).await?;

        Ok(Self { db })
    }

    pub async fn insert_keys(&self, pdf_id: String, private_key: RsaPrivateKey, public_key_pem: String) -> Result<()> {
        let private_key_pem = private_key.to_pkcs8_pem(LineEnding::LF)?.to_string();

        let sql = "INSERT INTO pdf (record_num, public_key, private_key, file_path) VALUES ($1, $2, $3, '')";
        
        sqlx::query(sql)
            .bind(pdf_id)
            .bind(public_key_pem)
            .bind(private_key_pem)
            .execute(self.db.pool())
            .await?;
            
        Ok(())
    }

    pub async fn get_private_key(&self, pdf_id: &str) -> Result<RsaPrivateKey> {
        let sql = "SELECT private_key FROM pdf WHERE record_num = $1";
        
        let row: (String,) = sqlx::query_as::<_, (String,)>(sql)
            .bind(pdf_id)
            .fetch_one(self.db.pool())
            .await
            .map_err(|e| anyhow!("Failed to fetch private key: {}", e))?;

        let priv_key_pem = row.0;
        let priv_key = RsaPrivateKey::from_pkcs8_pem(&priv_key_pem)?;
        
        Ok(priv_key)
    }

    pub async fn update_file_path(&self, pdf_id: &str, file_path: &str) -> Result<()> {
        let sql = "UPDATE pdf SET file_path = $1, description = 'Received' WHERE record_num = $2";

        let result = sqlx::query(sql)
            .bind(file_path)
            .bind(pdf_id)
            .execute(self.db.pool())
            .await?;

        if result.rows_affected() == 0 {
            return Err(anyhow!("PDF Record not found to update"));
        }

        Ok(())
    }

    pub async fn get_case_file_path(&self, case_code: &str) -> Result<String> {
        let sql = "SELECT file_path FROM pdf WHERE record_num = $1";

        let row: (String,) = sqlx::query_as::<_, (String,)>(sql)
            .bind(case_code)
            .fetch_one(self.db.pool())
            .await
            .map_err(|e| anyhow!("Failed to fetch file path: {}", e))?;

        if row.0.trim().is_empty() {
            return Err(anyhow!("File path is empty for case"));
        }

        Ok(row.0)
    }

    pub async fn list_cases(&self) -> Result<Vec<CaseSummary>> {
        #[derive(FromRow)]
        struct CaseRow {
            record_num: String,
            file_path: String,
            description: Option<String>,
            created_at: Option<chrono::NaiveDateTime>,
        }

        let sql = "SELECT record_num, file_path, description, created_at FROM pdf ORDER BY created_at DESC";

        let rows: Vec<CaseRow> = sqlx::query_as(sql)
            .fetch_all(self.db.pool())
            .await
            .map_err(|e| anyhow!("Failed to fetch cases: {}", e))?;

        Ok(rows
            .into_iter()
            .map(|row| CaseSummary {
                case_code: row.record_num,
                file_path: row.file_path,
                description: row.description,
                created_at: row.created_at,
            })
            .collect())
    }
}


