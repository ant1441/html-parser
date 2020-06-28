use derive_more::{Constructor, From};

use crate::dom::Node;

#[derive(Clone, Constructor, Debug, Default, Eq, From, PartialEq)]
pub struct DocumentFragment {
    children: Vec<Node>,
}

impl DocumentFragment {
    pub fn len(&self) -> usize {
        self.children.iter().map(|c| c.len()).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.children.is_empty()
    }
}
