use crate::prelude::*;
use crate::domain::EncryptedPackage;

#[derive(Clone)]
pub struct KeyRecord {
    pub pdf_id: String,
    pub private_key: RsaPrivateKey,
    pub public_key_pem: String,
    pub complete: bool,
    pub encrypted_pkg: Option<EncryptedPackage>,
}

#[derive(Clone)]
pub struct Database {
    // In-memory store: PDF_ID -> Record
    store: Arc<Mutex<HashMap<String, KeyRecord>>>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            store: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn insert_keys(&self, pdf_id: String, private_key: RsaPrivateKey, public_key_pem: String) -> Result<()> {
        let mut store = self.store.lock().map_err(|_| anyhow!("Poisoned lock"))?;
        store.insert(pdf_id.clone(), KeyRecord {
            pdf_id,
            private_key,
            public_key_pem,
            complete: false,
            encrypted_pkg: None,
        });
        Ok(())
    }

    pub fn get_private_key(&self, pdf_id: &str) -> Result<RsaPrivateKey> {
        let store = self.store.lock().map_err(|_| anyhow!("Poisoned lock"))?;
        let record = store.get(pdf_id).ok_or_else(|| anyhow!("PDF ID not found"))?;
        Ok(record.private_key.clone())
    }

    pub fn update_with_package(&self, pdf_id: &str, pkg: EncryptedPackage) -> Result<()> {
        let mut store = self.store.lock().map_err(|_| anyhow!("Poisoned lock"))?;
        if let Some(record) = store.get_mut(pdf_id) {
            record.encrypted_pkg = Some(pkg);
            record.complete = true;
            Ok(())
        } else {
            Err(anyhow!("PDF ID not found"))
        }
    }
}
