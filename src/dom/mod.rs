#![allow(dead_code)]

use derive_more::{Constructor, From};
use serde::{Deserialize, Serialize};

#[derive(Clone, Constructor, Debug, Default, Deserialize, Eq, From, Hash, PartialEq, Serialize)]
pub struct Document {
}

impl Document {
    pub(crate) fn add_document_type(&mut self, _document_type: DocumentType) { todo!("Document::add_document_type") }
    pub(crate) fn set_mode(&mut self, _mode: &str) { todo!("Document::set_mode") }
}

#[derive(Clone, Constructor, Debug, Default, Deserialize, Eq, From, Hash, PartialEq, Serialize)]
pub struct DocumentType {
    name: String,
    public_id: String,
    system_id: String,
}
