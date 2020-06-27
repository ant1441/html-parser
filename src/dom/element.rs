use std::cell::RefCell;
use std::rc::Rc;

use derive_more::{From, Deref, DerefMut};

use super::{Comment, ProcessingInstruction, Text};
use crate::{dom::Namespace, tokenizer::TagName};

#[derive(Clone, Debug, Eq, From, PartialEq)]
pub enum ElementChildNode {
    Element(Rc<RefCell<Element>>),
    Text(Rc<RefCell<Text>>),
    ProcessingInstruction(Rc<RefCell<ProcessingInstruction>>),
    Comment(Rc<RefCell<Comment>>),
}

impl ElementChildNode {
    pub fn len(&self) -> usize {
        match self {
            ElementChildNode::Element(e) => e.borrow().len(),
            ElementChildNode::Text(_) => 1,
            ElementChildNode::ProcessingInstruction(_) => 1,
            ElementChildNode::Comment(_) => 1,
        }
    }
}

#[derive(Clone, Debug, Eq, From, PartialEq, Deref, DerefMut)]
pub struct Element {
    pub name: TagName,
    pub namespace: Namespace,
    #[deref]
    #[deref_mut]
    children: Vec<ElementChildNode>,
}

impl Element {
    pub fn name(&self) -> &TagName {
        &self.name
    }

    pub fn namespace(&self) -> Namespace {
        self.namespace
    }

    pub fn new_html(name: TagName) -> Rc<RefCell<Self>> {
        let elem = Element {
            name,
            namespace: Default::default(),
            children: Vec::new(),
        };
        Rc::new(RefCell::new(elem))
    }

    pub fn is_html(&self) -> bool {
        self.namespace == Namespace::HTML
    }

    /// https://html.spec.whatwg.org/multipage/parsing.html#mathml-text-integration-point
    pub fn is_mathml_text_integration_point(&self) -> bool {
        match (self.namespace, self.name()) {
            (Namespace::MathML, TagName::Mi) => true,
            (Namespace::MathML, TagName::Mo) => true,
            (Namespace::MathML, TagName::Mn) => true,
            (Namespace::MathML, TagName::Ms) => true,
            (Namespace::MathML, TagName::Mtext) => true,
            _ => false,
        }
    }

    /// https://html.spec.whatwg.org/multipage/parsing.html#html-integration-point
    pub fn is_html_integration_point(&self) -> bool {
        match (self.namespace, self.name()) {
            (Namespace::MathML, TagName::AnnotationXml) => todo!(),
            (Namespace::SVG, TagName::ForeignObject) => true,
            (Namespace::SVG, TagName::Desc) => true,
            (Namespace::SVG, TagName::Title) => true,
            _ => false,
        }
    }

    #[allow(dead_code)]
    pub fn category(&self) -> Category {
        if self.namespace == Namespace::HTML {
            match self.name {
                TagName::Address
                | TagName::Applet
                | TagName::Area
                | TagName::Article
                | TagName::Aside
                | TagName::Base
                | TagName::Basefont
                | TagName::Bgsound
                | TagName::Blockquote
                | TagName::Body
                | TagName::Br
                | TagName::Button
                | TagName::Caption
                | TagName::Center
                | TagName::Col
                | TagName::Colgroup
                | TagName::Dd
                | TagName::Details
                | TagName::Dir
                | TagName::Div
                | TagName::Dl
                | TagName::Dt
                | TagName::Embed
                | TagName::Fieldset
                | TagName::Figcaption
                | TagName::Figure
                | TagName::Footer
                | TagName::Form
                | TagName::Frame
                | TagName::Frameset
                | TagName::H1
                | TagName::H2
                | TagName::H3
                | TagName::H4
                | TagName::H5
                | TagName::H6
                | TagName::Head
                | TagName::Header
                | TagName::Hgroup
                | TagName::Hr
                | TagName::Html
                | TagName::Iframe
                | TagName::Img
                | TagName::Input
                | TagName::Keygen
                | TagName::Li
                | TagName::Link
                | TagName::Listing
                | TagName::Main
                | TagName::Marquee
                | TagName::Menu
                | TagName::Meta
                | TagName::Nav
                | TagName::Noembed
                | TagName::Noframes
                | TagName::Noscript
                | TagName::Object
                | TagName::Ol
                | TagName::P
                | TagName::Param
                | TagName::Plaintext
                | TagName::Pre
                | TagName::Script
                | TagName::Section
                | TagName::Select
                | TagName::Source
                | TagName::Style
                | TagName::Summary
                | TagName::Table
                | TagName::Tbody
                | TagName::Td
                | TagName::Template
                | TagName::Textarea
                | TagName::Tfoot
                | TagName::Th
                | TagName::Thead
                | TagName::Title
                | TagName::Tr
                | TagName::Track
                | TagName::Ul
                | TagName::Wbr
                | TagName::Xmp => Category::Special,

                TagName::A
                | TagName::B
                | TagName::Big
                | TagName::Code
                | TagName::Em
                | TagName::Font
                | TagName::I
                | TagName::Nobr
                | TagName::S
                | TagName::Small
                | TagName::Strike
                | TagName::Strong
                | TagName::Tt
                | TagName::U => Category::Formatting,

                _ => Category::Ordinary,
            }
        } else if self.namespace == Namespace::MathML {
            match self.name {
                TagName::Mi
                | TagName::Mo
                | TagName::Mn
                | TagName::Ms
                | TagName::Mtext
                | TagName::AnnotationXml => Category::Special,

                _ => Category::Ordinary,
            }
        } else if self.namespace == Namespace::SVG {
            match self.name {
                TagName::ForeignObject | TagName::Desc | TagName::Time => Category::Special,

                _ => Category::Ordinary,
            }
        } else {
            Category::Ordinary
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, From, PartialEq)]
pub enum Category {
    Special,
    Formatting,
    Ordinary,
}
