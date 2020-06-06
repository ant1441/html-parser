use std::fmt;

use derive_more::From;

#[derive(Clone, Debug, PartialEq, Eq, From)]
pub enum Token {
    Doctype(Doctype),
    StartTag(StartTag),
    EndTag(EndTag),

    Comment(String),
    Character(char),
    // Internally, we'll collapse multiple 'Character's into Characters
    #[from(ignore)]
    Characters(String),
    Eof,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Attribute {
    pub name: String,
    pub value: String,
    duplicate: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Doctype {
    pub name: Option<String>,
    pub public_identifier: Option<String>,
    pub system_identifier: Option<String>,
    pub force_quirks: ForceQuirksFlag,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct StartTag {
    pub name: String,
    pub self_closing: SelfClosingFlag,
    pub attributes: Vec<Attribute>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct EndTag {
    pub name: String,
    pub self_closing: SelfClosingFlag,
    pub attributes: Vec<Attribute>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SelfClosingFlag {
    Set,
    Unset,
}

impl Default for SelfClosingFlag {
    fn default() -> Self {
        SelfClosingFlag::Unset
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ForceQuirksFlag {
    Off,
    On,
}

impl Default for ForceQuirksFlag {
    fn default() -> Self {
        ForceQuirksFlag::Off
    }
}

impl Token {
    pub(crate) fn push(&mut self, c: char) {
        use Token::*;
        match self {
            Doctype(t) => t.push(c),
            StartTag(t) => t.push(c),
            EndTag(t) => t.push(c),
            Comment(t) => t.push(c),
            Characters(t) => t.push(c),
            Character(_) | Eof => panic!("Cannot push on {:?}", self),
        }
    }

    pub(crate) fn push_str(&mut self, string: &str) {
        use Token::*;
        match self {
            Doctype(t) => t.push_str(string),
            StartTag(t) => t.push_str(string),
            EndTag(t) => t.push_str(string),
            Comment(t) => t.push_str(string),
            Characters(t) => t.push_str(string),
            Character(_) | Eof => panic!("Cannot push on {:?}", self),
        }
    }

    pub(crate) fn push_token(&mut self, token: Token) {
        use Token::*;
        match token {
            Character(c) => self.push(c),
            _ => panic!("Cannot push_tokens on {:?}", self),
        }
    }

    pub(crate) fn is_character(&self) -> bool {
        match self {
            Token::Character(_) => true,
            _ => false,
        }
    }

    pub(crate) fn is_eof(&self) -> bool {
        match self {
            Token::Eof => true,
            _ => false,
        }
    }

    pub(crate) fn is_appropriate_end_tag(&self) -> bool {
        match self {
            Token::EndTag(t) => t.is_appropriate_end_tag(),
            _ => false,
        }
    }

    pub(crate) fn add_attribute(&mut self, name: String, value: String) {
        match self {
            Token::StartTag(t) => t.add_attribute(name, value),
            Token::EndTag(t) => t.add_attribute(name, value),
            _ => panic!("Cannot add_attribute on {:?}", self),
        }
    }

    pub(crate) fn current_attribute(&mut self) -> Option<&Attribute> {
        match self {
            Token::StartTag(t) => t.current_attribute(),
            Token::EndTag(t) => t.current_attribute(),
            _ => panic!("Cannot current_attribute on {:?}", self),
        }
    }

    pub(crate) fn current_attribute_mut(&mut self) -> Option<&mut Attribute> {
        match self {
            Token::StartTag(t) => t.current_attribute_mut(),
            Token::EndTag(t) => t.current_attribute_mut(),
            _ => panic!("Cannot current_attribute_mut on {:?}", self),
        }
    }

    pub(crate) fn set_force_quirks(&mut self, f: ForceQuirksFlag) {
        match self {
            Token::Doctype(t) => t.set_force_quirks(f),
            _ => panic!("Cannot set_force_quirks on {:?}", self),
        }
    }

    pub(crate) fn set_self_closing(&mut self, f: SelfClosingFlag) {
        match self {
            Token::StartTag(t) => t.set_self_closing(f),
            Token::EndTag(t) => t.set_self_closing(f),
            _ => panic!("Cannot set_self_closing on {:?}", self),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Token::*;
        match self {
            Doctype(t) => write!(f, "{}", t),
            StartTag(t) => write!(f, "{}", t),
            EndTag(t) => write!(f, "{}", t),
            Comment(t) => write!(f, "Comment({})", t),
            Characters(t) => write!(f, "Characters({:?})", t),
            Character(t) => write!(f, "Character({})", t),
            Eof => write!(f, "Token(EOF)"),
        }
    }
}

impl Doctype {
    pub(crate) fn push(&mut self, c: char) {
        if self.name.is_none() {
            panic!("Cannot push to token::Docktype with no name");
        }
        let mut name = self.name.take().unwrap();
        name.push(c);
        self.name.replace(name);
    }

    pub(crate) fn push_str(&mut self, string: &str) {
        if self.name.is_none() {
            panic!("Cannot push to token::Docktype with no name");
        }
        let mut name = self.name.take().unwrap();
        name.push_str(string);
        self.name.replace(name);
    }

    pub(crate) fn set_force_quirks(&mut self, f: ForceQuirksFlag) {
        self.force_quirks = f
    }
}

impl fmt::Display for Doctype {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        assert!(self.name.is_some(), "`tokenizer::token::Doctype` doesn't implement `std::fmt::Display` when `name` is not set");
        assert!(self.public_identifier.is_none(), "`tokenizer::token::Doctype` doesn't implement `std::fmt::Display` when `public_identifier` is set");
        assert!(self.system_identifier.is_none(), "`tokenizer::token::Doctype` doesn't implement `std::fmt::Display` when `system_identifier` is set");
        assert!(self.force_quirks == ForceQuirksFlag::Off, "`tokenizer::token::Doctype` doesn't implement `std::fmt::Display` when `force_quirks` is not Off");

        if let Some(ref name) = self.name {
            write!(f, "<!doctype {}>", name)
        } else {
            unreachable!()
        }
    }
}

impl StartTag {
    pub(crate) fn push(&mut self, c: char) {
        self.name.push(c);
    }

    pub(crate) fn push_str(&mut self, string: &str) {
        self.name.push_str(string);
    }

    pub(crate) fn add_attribute(&mut self, name: String, value: String) {
        self.attributes.push(Attribute {
            name,
            value,
            duplicate: false,
        })
    }

    pub(crate) fn attributes_iter(&mut self) -> impl Iterator<Item = &Attribute> + '_ {
        self.attributes.iter()
    }

    pub(crate) fn current_attribute(&mut self) -> Option<&Attribute> {
        self.attributes.last()
    }

    pub(crate) fn current_attribute_mut(&mut self) -> Option<&mut Attribute> {
        self.attributes.last_mut()
    }

    pub(crate) fn set_self_closing(&mut self, f: SelfClosingFlag) {
        self.self_closing = f
    }
}

impl fmt::Display for StartTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{}", self.name)?;
        for attribute in self.attributes.iter() {
            write!(f, " {}", attribute.name)?;
            if !attribute.value.is_empty() {
                write!(f, "=\"{}\"", attribute.value)?;
            }
        }
        if self.self_closing == SelfClosingFlag::Set {
            write!(f, "/>")
        } else {
            write!(f, ">")
        }
    }
}

impl EndTag {
    pub(crate) fn push(&mut self, c: char) {
        self.name.push(c);
    }

    pub(crate) fn push_str(&mut self, string: &str) {
        self.name.push_str(string);
    }

    pub(crate) fn add_attribute(&mut self, name: String, value: String) {
        self.attributes.push(Attribute {
            name,
            value,
            duplicate: false,
        })
    }

    pub(crate) fn attributes_iter(&mut self) -> impl Iterator<Item = &Attribute> + '_ {
        self.attributes.iter()
    }

    pub(crate) fn current_attribute(&mut self) -> Option<&Attribute> {
        self.attributes.last()
    }

    pub(crate) fn current_attribute_mut(&mut self) -> Option<&mut Attribute> {
        self.attributes.last_mut()
    }

    // An appropriate end tag token is an end tag token whose tag name matches
    // the tag name of the last start tag to have been emitted from this
    // tokenizer, if any.
    // If no start tag has been emitted from this tokenizer, then no end tag
    // token is appropriate.
    pub(crate) fn is_appropriate_end_tag(&self) -> bool {
        // TODO
        true
    }

    pub(crate) fn set_self_closing(&mut self, f: SelfClosingFlag) {
        self.self_closing = f
    }
}

impl fmt::Display for EndTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        assert!(self.self_closing == SelfClosingFlag::Unset, "`tokenizer::token::EndTag` doesn't implement `std::fmt::Display` when `self_closing` is set");

        write!(f, "</{}", self.name)?;
        for attribute in self.attributes.iter() {
            write!(f, " {}", attribute.name)?;
            if !attribute.value.is_empty() {
                write!(f, "=\"{}\"", attribute.value)?;
            }
        }
        write!(f, ">")
    }
}

impl Attribute {
    pub(crate) fn push_name(&mut self, c: char) {
        self.name.push(c);
    }

    pub(crate) fn push_value(&mut self, c: char) {
        self.value.push(c);
    }

    pub(crate) fn set_duplicate(&mut self) {
        self.duplicate = true
    }
}
