use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Clone, Display, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum TagName {
    A,
    Abbr,
    Address,
    Applet,
    Area,
    Article,
    Aside,
    Audio,
    B,
    Base,
    Basefont,
    Bdi,
    Bdo,
    Bgsound,
    Big,
    Blink,
    Blockquote,
    Body,
    Br,
    Button,
    Canvas,
    Caption,
    Center,
    Cite,
    Code,
    Col,
    Colgroup,
    Data,
    Datalist,
    Dd,
    Del,
    Details,
    Dfn,
    Dialog,
    Dir,
    Div,
    Dl,
    Dt,
    Em,
    Embed,
    Fieldset,
    Figcaption,
    Figure,
    Font,
    Footer,
    Form,
    Frame,
    Frameset,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Head,
    Header,
    Hgroup,
    Hr,
    Html,
    I,
    Iframe,
    Img,
    Input,
    Ins,
    Kbd,
    Keygen,
    Label,
    Legend,
    Li,
    Link,
    Listing,
    Main,
    Map,
    Mark,
    Marquee,
    Math,
    Menu,
    Menuitem,
    Meta,
    Meter,
    Nav,
    Nobr,
    Noembed,
    Noframes,
    Noscript,
    Object,
    Ol,
    Optgroup,
    Option,
    Output,
    P,
    Param,
    Picture,
    Plaintext,
    Pre,
    Progress,
    Q,
    Rb,
    Rp,
    Rt,
    Rtc,
    Ruby,
    S,
    Samp,
    Script,
    Section,
    Select,
    Slot,
    Small,
    Source,
    Span,
    Strike,
    Strong,
    Style,
    Sub,
    Summary,
    Sup,
    Svg,
    Table,
    Tbody,
    Td,
    Template,
    Textarea,
    Tfoot,
    Th,
    Thead,
    Time,
    Title,
    Tr,
    Track,
    Tt,
    U,
    Ul,
    Var,
    Video,
    Wbr,
    Xmp,

    Other(String),
}

impl TagName {
    pub(super) fn finalize(&mut self) -> Self {
        if let TagName::Other(s) = self {
            s.parse().unwrap()
        } else {
            self.clone()
        }
    }

    pub(super) fn push(&mut self, c: char) {
        if let TagName::Other(ref mut s) = self {
            s.push(c)
        } else {
            panic!("Attempted to push on finalized TagName")
        }
    }

    pub(super) fn push_str(&mut self, string: &str) {
        if let TagName::Other(ref mut s) = self {
            s.push_str(string)
        } else {
            panic!("Attempted to push_str on finalized TagName")
        }
    }
}

impl Default for TagName {
    fn default() -> Self {
        TagName::Other(Default::default())
    }
}

impl std::str::FromStr for TagName {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "abbr" => Ok(TagName::Abbr),
            "address" => Ok(TagName::Address),
            "a" => Ok(TagName::A),
            "applet" => Ok(TagName::Applet),
            "area" => Ok(TagName::Area),
            "article" => Ok(TagName::Article),
            "aside" => Ok(TagName::Aside),
            "audio" => Ok(TagName::Audio),
            "basefont" => Ok(TagName::Basefont),
            "base" => Ok(TagName::Base),
            "bdi" => Ok(TagName::Bdi),
            "bdo" => Ok(TagName::Bdo),
            "bgsound" => Ok(TagName::Bgsound),
            "big" => Ok(TagName::Big),
            "blink" => Ok(TagName::Blink),
            "blockquote" => Ok(TagName::Blockquote),
            "body" => Ok(TagName::Body),
            "b" => Ok(TagName::B),
            "br" => Ok(TagName::Br),
            "button" => Ok(TagName::Button),
            "canvas" => Ok(TagName::Canvas),
            "caption" => Ok(TagName::Caption),
            "center" => Ok(TagName::Center),
            "cite" => Ok(TagName::Cite),
            "code" => Ok(TagName::Code),
            "colgroup" => Ok(TagName::Colgroup),
            "col" => Ok(TagName::Col),
            "datalist" => Ok(TagName::Datalist),
            "data" => Ok(TagName::Data),
            "dd" => Ok(TagName::Dd),
            "del" => Ok(TagName::Del),
            "details" => Ok(TagName::Details),
            "dfn" => Ok(TagName::Dfn),
            "dialog" => Ok(TagName::Dialog),
            "dir" => Ok(TagName::Dir),
            "div" => Ok(TagName::Div),
            "dl" => Ok(TagName::Dl),
            "dt" => Ok(TagName::Dt),
            "embed" => Ok(TagName::Embed),
            "em" => Ok(TagName::Em),
            "fieldset" => Ok(TagName::Fieldset),
            "figcaption" => Ok(TagName::Figcaption),
            "figure" => Ok(TagName::Figure),
            "font" => Ok(TagName::Font),
            "footer" => Ok(TagName::Footer),
            "form" => Ok(TagName::Form),
            "frame" => Ok(TagName::Frame),
            "frameset" => Ok(TagName::Frameset),
            "h1" => Ok(TagName::H1),
            "h2" => Ok(TagName::H2),
            "h3" => Ok(TagName::H3),
            "h4" => Ok(TagName::H4),
            "h5" => Ok(TagName::H5),
            "h6" => Ok(TagName::H6),
            "header" => Ok(TagName::Header),
            "head" => Ok(TagName::Head),
            "hgroup" => Ok(TagName::Hgroup),
            "hr" => Ok(TagName::Hr),
            "html" => Ok(TagName::Html),
            "iframe" => Ok(TagName::Iframe),
            "img" => Ok(TagName::Img),
            "input" => Ok(TagName::Input),
            "ins" => Ok(TagName::Ins),
            "i" => Ok(TagName::I),
            "kbd" => Ok(TagName::Kbd),
            "keygen" => Ok(TagName::Keygen),
            "label" => Ok(TagName::Label),
            "legend" => Ok(TagName::Legend),
            "link" => Ok(TagName::Link),
            "li" => Ok(TagName::Li),
            "listing" => Ok(TagName::Listing),
            "main" => Ok(TagName::Main),
            "map" => Ok(TagName::Map),
            "mark" => Ok(TagName::Mark),
            "marquee" => Ok(TagName::Marquee),
            "math" => Ok(TagName::Math),
            "menuitem" => Ok(TagName::Menuitem),
            "menu" => Ok(TagName::Menu),
            "meta" => Ok(TagName::Meta),
            "meter" => Ok(TagName::Meter),
            "nav" => Ok(TagName::Nav),
            "nobr" => Ok(TagName::Nobr),
            "noembed" => Ok(TagName::Noembed),
            "noframes" => Ok(TagName::Noframes),
            "noscript" => Ok(TagName::Noscript),
            "object" => Ok(TagName::Object),
            "ol" => Ok(TagName::Ol),
            "optgroup" => Ok(TagName::Optgroup),
            "option" => Ok(TagName::Option),
            "output" => Ok(TagName::Output),
            "param" => Ok(TagName::Param),
            "picture" => Ok(TagName::Picture),
            "plaintext" => Ok(TagName::Plaintext),
            "p" => Ok(TagName::P),
            "pre" => Ok(TagName::Pre),
            "progress" => Ok(TagName::Progress),
            "q" => Ok(TagName::Q),
            "rb" => Ok(TagName::Rb),
            "rp" => Ok(TagName::Rp),
            "rtc" => Ok(TagName::Rtc),
            "rt" => Ok(TagName::Rt),
            "ruby" => Ok(TagName::Ruby),
            "samp" => Ok(TagName::Samp),
            "script" => Ok(TagName::Script),
            "section" => Ok(TagName::Section),
            "select" => Ok(TagName::Select),
            "slot" => Ok(TagName::Slot),
            "small" => Ok(TagName::Small),
            "s" => Ok(TagName::S),
            "source" => Ok(TagName::Source),
            "span" => Ok(TagName::Span),
            "strike" => Ok(TagName::Strike),
            "strong" => Ok(TagName::Strong),
            "style" => Ok(TagName::Style),
            "sub" => Ok(TagName::Sub),
            "summary" => Ok(TagName::Summary),
            "sup" => Ok(TagName::Sup),
            "svg" => Ok(TagName::Svg),
            "table" => Ok(TagName::Table),
            "tbody" => Ok(TagName::Tbody),
            "td" => Ok(TagName::Td),
            "template" => Ok(TagName::Template),
            "textarea" => Ok(TagName::Textarea),
            "tfoot" => Ok(TagName::Tfoot),
            "thead" => Ok(TagName::Thead),
            "th" => Ok(TagName::Th),
            "time" => Ok(TagName::Time),
            "title" => Ok(TagName::Title),
            "track" => Ok(TagName::Track),
            "tr" => Ok(TagName::Tr),
            "tt" => Ok(TagName::Tt),
            "ul" => Ok(TagName::Ul),
            "u" => Ok(TagName::U),
            "var" => Ok(TagName::Var),
            "video" => Ok(TagName::Video),
            "wbr" => Ok(TagName::Wbr),
            "xmp" => Ok(TagName::Xmp),

            _ => Ok(TagName::Other(s.to_string())),
        }
    }
}
