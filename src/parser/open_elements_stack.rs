use std::{cell::RefCell, rc::Rc};

use derive_more::{Deref, DerefMut};

use crate::{dom, tokenizer::TagName};

#[derive(Debug, Default, PartialEq, Eq, Deref, DerefMut)]
pub(super) struct OpenElementsStack {
    stack: Vec<Rc<RefCell<dom::Element>>>,
}

impl OpenElementsStack {
    pub(crate) fn new() -> Self {
        OpenElementsStack { stack: Vec::new() }
    }

    pub(crate) fn contains_element(&self, name: &TagName) -> bool {
        self.stack
            .iter()
            .by_ref()
            .filter(|e| e.borrow().name() == name)
            .count()
            > 0
    }
}
