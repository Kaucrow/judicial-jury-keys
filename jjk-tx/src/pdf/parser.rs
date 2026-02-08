use crate::prelude::*;
use super::PdfData;

pub struct PdfParser {}

impl PdfParser {
    pub async fn parse(
        mut field: actix_multipart::Field,
    ) -> anyhow::Result<PdfData> {
        debug!("Parsing PDF...");

        // Read the file content into memory
        let mut file_bytes = Vec::new();
        while let Some(chunk) = field.next().await {
            let data = chunk.map_err(|_| anyhow!("Failed to load PDF into memory"))?;
            file_bytes.extend_from_slice(&data);
        }

        // Load the PDF from the memory buffer
        let doc = lopdf::Document::load_from(Cursor::new(&file_bytes))?;

        // Most metadata is stored in the "Info" dictionary
        let info_ref = doc.trailer.get(b"Info")?
            .as_reference()?;

        let info = doc.get_dictionary(info_ref)?;

        // Helper to get metadata fields
        let get_field = |key: &[u8]| {
            info.get(key)
                .ok()
                .and_then(|obj| {
                    match obj {
                        lopdf::Object::String(bytes, _) => Some(bytes.as_slice()),
                        lopdf::Object::Name(bytes) => Some(bytes.as_slice()),
                        _ => None,
                    }
                })
                .map(|bytes| String::from_utf8_lossy(bytes).into_owned())
                .unwrap_or_else(|| "Unknown".to_string())
        };

        let title = get_field(b"Title");
        let subject = get_field(b"Subject");
        let author = get_field(b"Author");
        let keywords = get_field(b"Keywords");

        debug!(
            "PDF parsed. Title: '{}', Subject: '{}', Author: '{}', Keywords: '{}'",
            title, subject, author, keywords
        );

        let data = PdfData {
            title,
            subject,
            author,
            keywords,
            file: file_bytes,
        };

        Ok(data)
    }
}