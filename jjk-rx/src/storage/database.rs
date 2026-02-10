use crate::prelude::*;
use crate::domain::EncryptedPackage; 
use crate::db::db_component::Db; 
use std::path::PathBuf;
use tokio::fs; 
use rsa::pkcs8::{EncodePrivateKey, DecodePrivateKey};

#[derive(Clone)]
pub struct Database {
    db: Db,
    storage_path: PathBuf,
}

impl Database {
    pub async fn connect(database_url: &str) -> Result<Self> {
        let db = Db::connect(database_url, 5).await?;
        let storage_path = PathBuf::from("./storage_files");
        if !storage_path.exists() {
            fs::create_dir_all(&storage_path).await?;
        }
        
        Ok(Self {
            db,
            storage_path,
        })
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

    pub async fn update_with_package(&self, pdf_id: &str, pkg: EncryptedPackage) -> Result<()> {
        let file_name = format!("{}.json", pdf_id);
        let file_path = self.storage_path.join(&file_name);
        
        let content = serde_json::to_string_pretty(&pkg)?;
        fs::write(&file_path, content).await?;

        let sql = "UPDATE pdf SET file_path = $1, description = 'Received' WHERE record_num = $2";
        
        let path_str = file_path.to_string_lossy().to_string();
        
        let result = sqlx::query(sql)
            .bind(path_str)
            .bind(pdf_id)
            .execute(self.db.pool())
            .await?;

        if result.rows_affected() == 0 {
            return Err(anyhow!("PDF Record not found to update"));
        }

        Ok(())
    }
}


