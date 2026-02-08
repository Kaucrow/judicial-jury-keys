pub mod parser;

pub use parser::PdfParser;

use crate::prelude::*;

#[derive(Serialize)]
pub struct PdfData {
    title: String,
    subject: String,
    author: String,
    keywords: String,
    file: Vec<u8>,
}