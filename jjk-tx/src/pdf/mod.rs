pub mod parser;

pub use parser::PdfParser;

use crate::prelude::*;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PdfData {
    title: String,
    subject: String,
    author: String,
    keywords: String,
    file: Vec<u8>,
}