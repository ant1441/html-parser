use derive_more::{Constructor, From};

use crate::dom::Node;

#[derive(Clone, Constructor, Debug, Default, Eq, From, PartialEq)]
pub struct DocumentFragment {
    children: Vec<Node>,
}

impl DocumentFragment {
    #[must_use]
    pub fn len(&self) -> usize {
        self.children.iter().map(Node::len).sum()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.children.is_empty()
    }
}
