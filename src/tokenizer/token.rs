use std::fmt;

use derive_more::From;

use crate::tokenizer::TagName;

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
    pub name: TagName,
    pub self_closing: SelfClosingFlag,
    pub attributes: Vec<Attribute>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct EndTag {
    pub name: TagName,
    pub self_closing: SelfClosingFlag,
    pub attributes: Vec<Attribute>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SelfClosingFlag {
    Set,
    Unset,
}

impl Default for SelfClosingFlag {
    fn default() -> Self {
        SelfClosingFlag::Unset
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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
    #[allow(clippy::match_same_arms)]
    pub(crate) fn push(&mut self, c: char) {
        match self {
            Token::Doctype(t) => t.push(c),
            Token::StartTag(t) => t.push(c),
            Token::EndTag(t) => t.push(c),
            Token::Comment(t) => t.push(c),
            Token::Characters(t) => t.push(c),
            Token::Character(_) | Token::Eof => panic!("Cannot push on {:?}", self),
        }
    }

    #[allow(clippy::match_same_arms)]
    pub(crate) fn push_str(&mut self, string: &str) {
        match self {
            Token::Doctype(t) => t.push_str(string),
            Token::StartTag(t) => t.push_str(string),
            Token::EndTag(t) => t.push_str(string),
            Token::Comment(t) => t.push_str(string),
            Token::Characters(t) => t.push_str(string),
            Token::Character(_) | Token::Eof => panic!("Cannot push on {:?}", self),
        }
    }

    pub(crate) fn push_token(&mut self, token: &Token) {
        match *token {
            Token::Character(c) => self.push(c),
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

    pub(crate) fn is_start_tag(&self) -> bool {
        match self {
            Token::StartTag(_) => true,
            _ => false,
        }
    }

    pub(crate) fn is_end_tag(&self) -> bool {
        match self {
            Token::EndTag(_) => true,
            _ => false,
        }
    }

    pub(crate) fn is_self_closing(&self) -> bool {
        match self {
            Token::StartTag(tag) => tag.is_self_closing(),
            Token::EndTag(tag) => tag.is_self_closing(),
            _ => false,
        }
    }

    pub(crate) fn tag_name(&self) -> Option<&TagName> {
        match self {
            Token::StartTag(tag) => Some(&tag.name),
            Token::EndTag(tag) => Some(&tag.name),
            _ => None,
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

    /// emitting should be called just before a Token is emitted
    pub(super) fn emitting(&mut self) {
        match self {
            Token::StartTag(t) => t.emitting(),
            Token::EndTag(t) => t.emitting(),
            _ => (),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Doctype(t) => write!(f, "{}", t),
            Token::StartTag(t) => write!(f, "{}", t),
            Token::EndTag(t) => write!(f, "{}", t),
            Token::Comment(t) => write!(f, "Comment({})", t),
            Token::Characters(t) => write!(f, "Characters({:?})", t),
            Token::Character(t) => write!(f, "Character({:?})", t),
            Token::Eof => write!(f, "Token(EOF)"),
        }
    }
}

impl From<&str> for Token {
    fn from(s: &str) -> Self {
        Token::from(s.to_string())
    }
}

impl Doctype {
    pub fn from_char(name: char) -> Self {
        Doctype {
            name: Some(name.to_string()),
            ..Doctype::default()
        }
    }

    pub(crate) fn push(&mut self, c: char) {
        if let Some(ref mut name) = self.name {
            name.push(c);
        } else {
            panic!("Cannot push to token::Docktype with no name");
        }
    }

    pub(crate) fn push_str(&mut self, string: &str) {
        if let Some(ref mut name) = self.name {
            name.push_str(string);
        } else {
            panic!("Cannot push to token::Docktype with no name");
        }
    }

    pub(crate) fn set_force_quirks(&mut self, f: ForceQuirksFlag) {
        self.force_quirks = f
    }

    pub(crate) fn is_force_quirks(&self) -> bool {
        self.force_quirks == ForceQuirksFlag::On
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

    pub(crate) fn is_self_closing(&self) -> bool {
        self.self_closing == SelfClosingFlag::Set
    }

    pub(crate) fn attributes_iter(&self) -> impl Iterator<Item = &Attribute> + '_ {
        self.attributes.iter()
    }

    pub(crate) fn attributes_iter_mut(&mut self) -> impl Iterator<Item = &mut Attribute> + '_ {
        self.attributes.iter_mut()
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

    /// emitting should be called just before a Token is emitted
    fn emitting(&mut self) {
        self.attributes.retain(|a| !a.duplicate);
        self.name = self.name.finalize();
    }
}

impl fmt::Display for StartTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{}", self.name)?;
        for attribute in &self.attributes {
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

    pub(crate) fn is_self_closing(&self) -> bool {
        self.self_closing == SelfClosingFlag::Set
    }

    pub(crate) fn attributes_iter(&self) -> impl Iterator<Item = &Attribute> + '_ {
        self.attributes.iter()
    }

    pub(crate) fn attributes_iter_mut(&mut self) -> impl Iterator<Item = &mut Attribute> + '_ {
        self.attributes.iter_mut()
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

    /// emitting should be called just before a Token is emitted
    fn emitting(&mut self) {
        self.name = self.name.finalize();
    }
}

impl fmt::Display for EndTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        assert!(self.self_closing == SelfClosingFlag::Unset, "`tokenizer::token::EndTag` doesn't implement `std::fmt::Display` when `self_closing` is set");

        write!(f, "</{}", self.name)?;
        for attribute in &self.attributes {
            write!(f, " {}", attribute.name)?;
            if !attribute.value.is_empty() {
                write!(f, "=\"{}\"", attribute.value)?;
            }
        }
        write!(f, ">")
    }
}

impl Attribute {
    #[cfg(test)]
    pub fn new(name: &str, value: &str, duplicate: bool) -> Self {
        Attribute {
            name: name.to_string(),
            value: value.to_string(),
            duplicate,
        }
    }

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
