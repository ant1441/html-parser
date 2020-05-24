use derive_more::From;

#[derive(Clone, Debug, PartialEq, Eq, From)]
pub enum Token {
    Doctype(Doctype),
    StartTag(StartTag),
    EndTag(EndTag),

    Comment(String),
    Character(char),
    EOF,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Attribute {
    pub name: String,
    pub value: String,
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
        match self {
            Token::Doctype(t) => t.push(c),
            Token::StartTag(t) => t.push(c),
            Token::EndTag(t) => t.push(c),
            Token::Comment(t) => t.push(c),
            _ => panic!("Cannot push on {:?}", self),
        }
    }

    pub(crate) fn is_appropriate_end_tag(&self) -> bool {
        match self {
            Token::EndTag(t) => t.is_appropriate_end_tag(),
            _ => false,
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
}

impl StartTag {
    pub(crate) fn push(&mut self, c: char) {
        self.name.push(c);
    }
}

impl EndTag {
    pub(crate) fn push(&mut self, c: char) {
        self.name.push(c);
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
}
