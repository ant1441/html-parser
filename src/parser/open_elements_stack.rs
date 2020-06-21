use derive_more::{Deref, DerefMut};

use crate::{
    dom,
    tokenizer::{TagName},
};

#[derive(Debug, Default, PartialEq, Eq, Deref, DerefMut)]
pub(super) struct OpenElementsStack {
    stack: Vec<dom::Element>
}

impl OpenElementsStack {
    pub fn contains_element(&self, name: &TagName) -> bool {
        self.stack.iter().by_ref().filter(|e| e.name() == name).count() > 0
    }
}
