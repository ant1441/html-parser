use std::{cell::RefCell, rc::Rc};

use derive_more::{Constructor, From};
use log::warn;
use serde::{Deserialize, Serialize};

use super::{Comment, DocumentType, Element, ProcessingInstruction};

#[derive(Clone, Constructor, Debug, Default, Eq, From, PartialEq)]
pub struct Document {
    first_children: Vec<DocumentChildNode>,
    document_type: Option<DocumentType>,
    second_children: Vec<DocumentChildNode>,
    element: Option<Rc<RefCell<Element>>>,
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
            + self.element.as_ref().map_or(0, |e| e.borrow().len())
            + self.third_children.len()
    }

    /// The document element of a document is the element whose parent is that document, if it exists, and null otherwise.
    ///
    /// ## Note
    /// Per the node tree constraints, there can be only one such element.
    pub fn document_element(&self) -> Option<Rc<RefCell<Element>>> {
        self.element.clone()
    }

    pub(crate) fn add_document_type(&mut self, document_type: DocumentType) {
        assert!(
            self.document_type.is_none(),
            "[{}::Document] Cannot add second document type",
            module_path!()
        );
        self.document_type = Some(document_type)
    }

    pub(crate) fn set_mode(&mut self, mode: &str) {
        warn!("[TODO] Document::set_mode({:?})", mode)
    }
    pub(crate) fn push_element(&mut self, elem: Rc<RefCell<Element>>) {
        if let Some(ref element) = self.element {
            let mut element = element.borrow_mut();
            element.push(elem.into())
        } else {
            self.element = Some(elem)
        }
    }
    pub(crate) fn push_comment(&mut self, elem: Comment) {
        warn!("[TODO] Document::push_comment({:?})", elem)
    }
}

trait DocumentInterface {
    // Dunno where this is in the IDL...
    fn doctype(&self) -> Option<&DocumentType>;
}

impl DocumentInterface for Document {
    fn doctype(&self) -> Option<&DocumentType> {
        self.document_type.as_ref()
    }
}
