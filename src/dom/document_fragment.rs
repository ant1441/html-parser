use derive_more::{Constructor, From};

use super::Node;

#[derive(Clone, Constructor, Debug, Default, Eq, From, PartialEq)]
pub struct DocumentFragment {
    children: Vec<Node>,
}

impl DocumentFragment {
    pub fn len(&self) -> usize {
        self.children.iter().map(|c| c.len()).sum()
    }
}
