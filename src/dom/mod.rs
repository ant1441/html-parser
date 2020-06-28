use derive_more::From;

mod comment;
mod document;
mod document_fragment;
mod namespace;
mod document_type;
mod element;
mod processing_instruction;
mod text;

pub use comment::Comment;
pub use namespace::Namespace;
pub use document::Document;
pub use document_fragment::DocumentFragment;
pub use document_type::DocumentType;
pub use element::{Element, ElementChildNode, Category};
pub use processing_instruction::ProcessingInstruction;
pub use text::Text;

#[derive(Clone, Debug, Eq, From, PartialEq)]
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

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
