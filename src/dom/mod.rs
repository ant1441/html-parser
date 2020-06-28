use derive_more::From;

pub mod comment;
pub mod document;
pub mod document_fragment;
pub mod document_type;
pub mod element;
pub mod namespace;
pub mod processing_instruction;
pub mod text;

pub use comment::Comment;
pub use document::Document;
pub use document_fragment::DocumentFragment;
pub use document_type::DocumentType;
pub use element::{Category, Element};
pub use namespace::Namespace;
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
