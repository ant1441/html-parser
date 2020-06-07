use derive_more::{Constructor, From};
use serde::{Deserialize, Serialize};

use super::Node;

#[derive(Clone, Constructor, Debug, Default, Deserialize, Eq, From, Hash, PartialEq, Serialize)]
pub struct DocumentFragment {
    children: Vec<Node>,
}

impl DocumentFragment {
    pub fn len(&self) -> usize {
        self.children.iter().map(|c| c.len()).sum()
    }
}
