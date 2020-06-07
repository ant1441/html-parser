use derive_more::From;
use serde::{Deserialize, Serialize};

mod comment;
mod document;
mod document_fragment;
mod document_type;
mod element;
mod processing_instruction;
mod text;

pub use comment::Comment;
pub use document::Document;
pub use document_fragment::DocumentFragment;
pub use document_type::DocumentType;
pub use element::Element;
pub use processing_instruction::ProcessingInstruction;
pub use text::Text;

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

impl Node {
    #[allow(dead_code)]
    fn len(&self) -> usize {
        match self {
            Node::DocumentType(_) => 0,
            Node::Text(inner) => inner.len(),
            Node::Comment(inner) => inner.len(),
            Node::ProcessingInstruction(inner) => inner.len(),
            Node::Document(inner) => inner.len(),
            Node::DocumentFragment(inner) => inner.len(),
            Node::Element(inner) => inner.len(),
        }
    }

    #[allow(dead_code)]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
