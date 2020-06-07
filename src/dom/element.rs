use derive_more::From;
use serde::{Deserialize, Serialize};

use super::{Comment, ProcessingInstruction, Text};

#[derive(Clone, Debug, Deserialize, Eq, From, Hash, PartialEq, Serialize)]
pub enum ElementChildNode {
    Element(Element),
    Text(Text),
    ProcessingInstruction(ProcessingInstruction),
    Comment(Comment),
}

impl ElementChildNode {
    pub fn len(&self) -> usize {
        match self {
            ElementChildNode::Element(e) => e.len(),
            ElementChildNode::Text(_) => 1,
            ElementChildNode::ProcessingInstruction(_) => 1,
            ElementChildNode::Comment(_) => 1,
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, From, Hash, PartialEq, Serialize)]
pub struct Element {
    name: String,
    children: Vec<ElementChildNode>,
}

impl Element {
    pub fn len(&self) -> usize {
        self.children.iter().map(|n| n.len()).sum()
    }

    pub fn new<N: ToString>(name: N) -> Self {
        Element {
            name: name.to_string(),
            children: Vec::new(),
        }
    }
}
