use std::{cell::RefCell, rc::Rc};

use derive_more::{Deref, DerefMut, From};

use crate::{dom, tokenizer::TagName};

#[derive(Debug, Default, PartialEq, Eq, Deref, DerefMut)]
pub(super) struct ListOfActiveFormattingElements {
    list: Vec<ActiveFormattingElementOrMarker>,
}

#[derive(Debug, PartialEq, Eq, From)]
pub(super) enum ActiveFormattingElementOrMarker {
    ActiveFormattingElement(Rc<RefCell<dom::Element>>),
    Marker,
}

impl ListOfActiveFormattingElements {
    pub(crate) fn new() -> Self {
        ListOfActiveFormattingElements { list: Vec::new() }
    }

    pub(crate) fn push_marker(&mut self) {
        self.list.push(ActiveFormattingElementOrMarker::Marker)
    }

    pub(crate) fn contains_element(&self, name: &TagName) -> bool {
        self.list
            .iter()
            .by_ref()
            .filter(|e| e.is_element(name))
            .count()
            > 0
    }
}

impl ActiveFormattingElementOrMarker {
    pub(crate) fn is_element(&self, name: &TagName) -> bool {
        match self {
            ActiveFormattingElementOrMarker::Marker => false,
            ActiveFormattingElementOrMarker::ActiveFormattingElement(e) => {
                e.borrow().name() == name
            }
        }
    }

    pub(crate) fn is_marker(&self) -> bool {
        match self {
            ActiveFormattingElementOrMarker::Marker => true,
            ActiveFormattingElementOrMarker::ActiveFormattingElement(_) => false,
        }
    }
}
