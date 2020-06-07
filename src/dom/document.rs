#![allow(dead_code)]

use derive_more::{Constructor, From};
use log::warn;
use serde::{Deserialize, Serialize};

use super::{Comment, DocumentType, Element, Node, ProcessingInstruction};

#[derive(Clone, Constructor, Debug, Default, Deserialize, Eq, From, Hash, PartialEq, Serialize)]
pub struct Document {
    first_children: Vec<DocumentChildNode>,
    document_type: Option<DocumentType>,
    second_children: Vec<DocumentChildNode>,
    element: Option<Element>,
    third_children: Vec<DocumentChildNode>,
}

#[derive(Clone, Debug, Deserialize, Eq, From, Hash, PartialEq, Serialize)]
pub enum DocumentChildNode {
    ProcessingInstruction(ProcessingInstruction),
    Comment(Comment),
}

impl Document {
    pub fn len(&self) -> usize {
        self.first_children.len()
            + if self.document_type.is_some() { 1 } else { 0 }
            + self.second_children.len()
            + self.element.as_ref().map_or(0, |e| e.len())
            + self.third_children.len()
    }

    /// The document element of a document is the element whose parent is that document, if it exists, and null otherwise.
    ///
    /// ## Note
    /// Per the node tree constraints, there can be only one such element.
    pub fn document_element(&self) -> Option<&Element> {
        self.element.as_ref()
    }

    pub(crate) fn add_document_type(&mut self, document_type: DocumentType) {
        assert!(
            self.document_type.is_none(),
            "[{}::Document] Cannot add second document type",
            module_path!()
        );
        self.document_type = Some(document_type)
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
