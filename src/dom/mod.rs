#![allow(dead_code)]

use derive_more::{Constructor, From};
use serde::{Deserialize, Serialize};
use log::warn;

#[derive(Clone, Debug, Deserialize, Eq, From, Hash, PartialEq, Serialize)]
pub enum Node {
    Document(Document),
    DocumentType(DocumentType),
    DocumentFragment(DocumentFragment),
    Element(Element),
    Text(Text),
    ProcessingInstruction(ProcessingInstruction),
    Comment(Comment),
}

#[derive(Clone, Constructor, Debug, Default, Deserialize, Eq, From, Hash, PartialEq, Serialize)]
pub struct Document {}

#[derive(Clone, Constructor, Debug, Default, Deserialize, Eq, From, Hash, PartialEq, Serialize)]
pub struct DocumentFragment {}

#[derive(Clone, Constructor, Debug, Default, Deserialize, Eq, From, Hash, PartialEq, Serialize)]
pub struct Element {
    name: String,
}

#[derive(Clone, Constructor, Debug, Default, Deserialize, Eq, From, Hash, PartialEq, Serialize)]
pub struct Text {}

#[derive(Clone, Constructor, Debug, Default, Deserialize, Eq, From, Hash, PartialEq, Serialize)]
pub struct ProcessingInstruction {}

#[derive(Clone, Constructor, Debug, Default, Deserialize, Eq, From, Hash, PartialEq, Serialize)]
pub struct DocumentType {
    name: String,
    public_id: String,
    system_id: String,
}

#[derive(Clone, Constructor, Debug, Default, Deserialize, Eq, From, Hash, PartialEq, Serialize)]
pub struct Comment {
    data: String,
}

impl Document {
    pub(crate) fn add_document_type(&mut self, document_type: DocumentType) {
        warn!("[TODO] Document::add_document_type({:?})", document_type)
    }
    pub(crate) fn set_head(&mut self, head: Element) {
        warn!("[TODO] Document::set_head({:?})", head)
    }
    pub(crate) fn set_mode(&mut self, mode: &str) {
        warn!("[TODO] Document::set_mode({:?})", mode)
    }
    pub(crate) fn push<N: Into<Node>>(&mut self, node: N) {
        warn!("[TODO] Document::push({:?})", node.into())
    }
}
