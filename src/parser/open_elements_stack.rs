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

    pub(crate) fn has_element_in_a_specific_scope(
        &self,
        target_node: &TagName,
        list: &[&TagName],
    ) -> bool {
        for node in self.iter().rev() {
            if &node.borrow().name == target_node {
                return true;
            } else if list.iter().any(|name| &&node.borrow().name == name) {
                return false;
            }
        }
        unreachable!()
    }

    pub(crate) fn has_a_particular_element_in_scope(&self, target_node: &TagName) -> bool {
        self.has_element_in_a_specific_scope(
            target_node,
            &[
                &TagName::Applet,
                &TagName::Caption,
                &TagName::Html,
                &TagName::Table,
                &TagName::Td,
                &TagName::Th,
                &TagName::Marquee,
                &TagName::Object,
                &TagName::Template,
                /* TODO: None HTML Elements:
                MathML mi
                MathML mo
                MathML mn
                MathML ms
                MathML mtext
                MathML annotation-xml
                SVG foreignObject
                SVG desc
                SVG title
                */
            ],
        )
    }

    pub(crate) fn has_an_element_in_scope(&self, target_node: &TagName) -> bool {
        self.has_a_particular_element_in_scope(target_node)
    }

    pub(crate) fn has_a_particular_element_in_list_item_scope(
        &self,
        target_node: &TagName,
    ) -> bool {
        self.has_element_in_a_specific_scope(
            target_node,
            &[
                &TagName::Applet,
                &TagName::Caption,
                &TagName::Html,
                &TagName::Table,
                &TagName::Td,
                &TagName::Th,
                &TagName::Marquee,
                &TagName::Object,
                &TagName::Template,
                /* TODO: None HTML Elements:
                MathML mi
                MathML mo
                MathML mn
                MathML ms
                MathML mtext
                MathML annotation-xml
                SVG foreignObject
                SVG desc
                SVG title
                */
                &TagName::Ol,
                &TagName::Ul,
            ],
        )
    }

    pub(crate) fn has_a_particular_element_in_button_scope(&self, target_node: &TagName) -> bool {
        self.has_element_in_a_specific_scope(
            target_node,
            &[
                &TagName::Applet,
                &TagName::Caption,
                &TagName::Html,
                &TagName::Table,
                &TagName::Td,
                &TagName::Th,
                &TagName::Marquee,
                &TagName::Object,
                &TagName::Template,
                /* TODO: None HTML Elements:
                MathML mi
                MathML mo
                MathML mn
                MathML ms
                MathML mtext
                MathML annotation-xml
                SVG foreignObject
                SVG desc
                SVG title
                */
                &TagName::Button,
            ],
        )
    }

    pub(crate) fn has_a_particular_element_in_table_scope(&self, target_node: &TagName) -> bool {
        self.has_element_in_a_specific_scope(
            target_node,
            &[&TagName::Html, &TagName::Table, &TagName::Template],
        )
    }

    pub(crate) fn has_a_particular_element_in_select_scope(&self, target_node: &TagName) -> bool {
        self.has_element_in_a_specific_scope(target_node, &[&TagName::Optgroup, &TagName::Option])
    }
}
