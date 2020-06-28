use std::{cell::RefCell, rc::Rc};

use derive_more::{Deref, DerefMut};
use log::trace;

use crate::{dom, tokenizer::TagName};

#[derive(Debug, Default, PartialEq, Eq, Deref, DerefMut)]
pub(super) struct OpenElementsStack {
    stack: Vec<Rc<RefCell<dom::Element>>>,
}

impl OpenElementsStack {
    pub(crate) fn new() -> Self {
        OpenElementsStack { stack: Vec::new() }
    }

    /// Returns true if any of the elements in the stack have the given `name`
    #[must_use]
    pub(crate) fn contains_element(&self, name: &TagName) -> bool {
        self.contains_any_elements(&[name])
    }

    /// Returns true if any of the elements in the stack have any of the given `names`
    #[must_use]
    pub(crate) fn contains_any_elements(&self, names: &[&TagName]) -> bool {
        self.stack
            .iter()
            .by_ref()
            .any(|e| names.iter().any(|name| &e.borrow().name() == name))
    }

    /// Pop elements off the stack until one of `names` has been popped
    pub(crate) fn pop_until(&mut self, names: &[&TagName]) {
        while let Some(e) = self.pop() {
            let elem = e.borrow();
            if names.iter().any(|name| &&elem.name == name) {
                break;
            }
            trace!("InBody: Popped {:?} off stack", elem);
        }
    }
}
