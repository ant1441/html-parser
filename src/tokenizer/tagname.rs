#![allow(clippy::match_same_arms)]

use log::warn;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, Deserialize, Serialize)]
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

    Mglyph,
    Malignmark,
    AnnotationXml,
    Mi,
    Mo,
    Mn,
    Ms,
    Mtext,
    ForeignObject,
    Desc,

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

    pub fn is_other(&self) -> bool {
        if let TagName::Other(_) = self {
            true
        } else {
            false
        }
    }
}

impl Default for TagName {
    fn default() -> Self {
        TagName::Other(String::default())
    }
}

impl std::str::FromStr for TagName {
    type Err = &'static str;

    #[allow(clippy::too_many_lines, clippy::enum_glob_use)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TagName::*;

        match s.to_lowercase().as_str() {
            "abbr" => Ok(Abbr),
            "address" => Ok(Address),
            "a" => Ok(A),
            "applet" => Ok(Applet),
            "area" => Ok(Area),
            "article" => Ok(Article),
            "aside" => Ok(Aside),
            "audio" => Ok(Audio),
            "basefont" => Ok(Basefont),
            "base" => Ok(Base),
            "bdi" => Ok(Bdi),
            "bdo" => Ok(Bdo),
            "bgsound" => Ok(Bgsound),
            "big" => Ok(Big),
            "blink" => Ok(Blink),
            "blockquote" => Ok(Blockquote),
            "body" => Ok(Body),
            "b" => Ok(B),
            "br" => Ok(Br),
            "button" => Ok(Button),
            "canvas" => Ok(Canvas),
            "caption" => Ok(Caption),
            "center" => Ok(Center),
            "cite" => Ok(Cite),
            "code" => Ok(Code),
            "colgroup" => Ok(Colgroup),
            "col" => Ok(Col),
            "datalist" => Ok(Datalist),
            "data" => Ok(Data),
            "dd" => Ok(Dd),
            "del" => Ok(Del),
            "details" => Ok(Details),
            "dfn" => Ok(Dfn),
            "dialog" => Ok(Dialog),
            "dir" => Ok(Dir),
            "div" => Ok(Div),
            "dl" => Ok(Dl),
            "dt" => Ok(Dt),
            "embed" => Ok(Embed),
            "em" => Ok(Em),
            "fieldset" => Ok(Fieldset),
            "figcaption" => Ok(Figcaption),
            "figure" => Ok(Figure),
            "font" => Ok(Font),
            "footer" => Ok(Footer),
            "form" => Ok(Form),
            "frame" => Ok(Frame),
            "frameset" => Ok(Frameset),
            "h1" => Ok(H1),
            "h2" => Ok(H2),
            "h3" => Ok(H3),
            "h4" => Ok(H4),
            "h5" => Ok(H5),
            "h6" => Ok(H6),
            "header" => Ok(Header),
            "head" => Ok(Head),
            "hgroup" => Ok(Hgroup),
            "hr" => Ok(Hr),
            "html" => Ok(Html),
            "iframe" => Ok(Iframe),
            "img" => Ok(Img),
            "input" => Ok(Input),
            "ins" => Ok(Ins),
            "i" => Ok(I),
            "kbd" => Ok(Kbd),
            "keygen" => Ok(Keygen),
            "label" => Ok(Label),
            "legend" => Ok(Legend),
            "link" => Ok(Link),
            "li" => Ok(Li),
            "listing" => Ok(Listing),
            "main" => Ok(Main),
            "map" => Ok(Map),
            "mark" => Ok(Mark),
            "marquee" => Ok(Marquee),
            "math" => Ok(Math),
            "menuitem" => Ok(Menuitem),
            "menu" => Ok(Menu),
            "meta" => Ok(Meta),
            "meter" => Ok(Meter),
            "nav" => Ok(Nav),
            "nobr" => Ok(Nobr),
            "noembed" => Ok(Noembed),
            "noframes" => Ok(Noframes),
            "noscript" => Ok(Noscript),
            "object" => Ok(Object),
            "ol" => Ok(Ol),
            "optgroup" => Ok(Optgroup),
            "option" => Ok(Option),
            "output" => Ok(Output),
            "param" => Ok(Param),
            "picture" => Ok(Picture),
            "plaintext" => Ok(Plaintext),
            "p" => Ok(P),
            "pre" => Ok(Pre),
            "progress" => Ok(Progress),
            "q" => Ok(Q),
            "rb" => Ok(Rb),
            "rp" => Ok(Rp),
            "rtc" => Ok(Rtc),
            "rt" => Ok(Rt),
            "ruby" => Ok(Ruby),
            "samp" => Ok(Samp),
            "script" => Ok(Script),
            "section" => Ok(Section),
            "select" => Ok(Select),
            "slot" => Ok(Slot),
            "small" => Ok(Small),
            "s" => Ok(S),
            "source" => Ok(Source),
            "span" => Ok(Span),
            "strike" => Ok(Strike),
            "strong" => Ok(Strong),
            "style" => Ok(Style),
            "sub" => Ok(Sub),
            "summary" => Ok(Summary),
            "sup" => Ok(Sup),
            "svg" => Ok(Svg),
            "table" => Ok(Table),
            "tbody" => Ok(Tbody),
            "td" => Ok(Td),
            "template" => Ok(Template),
            "textarea" => Ok(Textarea),
            "tfoot" => Ok(Tfoot),
            "thead" => Ok(Thead),
            "th" => Ok(Th),
            "time" => Ok(Time),
            "title" => Ok(Title),
            "track" => Ok(Track),
            "tr" => Ok(Tr),
            "tt" => Ok(Tt),
            "ul" => Ok(Ul),
            "u" => Ok(U),
            "var" => Ok(Var),
            "video" => Ok(Video),
            "wbr" => Ok(Wbr),
            "xmp" => Ok(Xmp),

            // MathML / SVG
            "mglyph" => Ok(Mglyph),
            "malignmark" => Ok(Malignmark),
            "annotation-xml" => Ok(AnnotationXml),
            "mi" => Ok(Mi),
            "mo" => Ok(Mo),
            "mn" => Ok(Mn),
            "ms" => Ok(Ms),
            "mtext" => Ok(Mtext),
            "foreignObject" => Ok(ForeignObject),
            "desc" => Ok(Desc),

            _ => {
                warn!("Unknown tag found: {}", s);
                Ok(Other(s.to_string()))
            }
        }
    }
}

impl PartialEq for TagName {
    #[allow(clippy::too_many_lines, clippy::enum_glob_use)]
    fn eq(&self, other: &Self) -> bool {
        use TagName::*;

        match (self, other) {
            (A, A) => true,
            (Abbr, Abbr) => true,
            (Address, Address) => true,
            (Applet, Applet) => true,
            (Area, Area) => true,
            (Article, Article) => true,
            (Aside, Aside) => true,
            (Audio, Audio) => true,
            (B, B) => true,
            (Base, Base) => true,
            (Basefont, Basefont) => true,
            (Bdi, Bdi) => true,
            (Bdo, Bdo) => true,
            (Bgsound, Bgsound) => true,
            (Big, Big) => true,
            (Blink, Blink) => true,
            (Blockquote, Blockquote) => true,
            (Body, Body) => true,
            (Br, Br) => true,
            (Button, Button) => true,
            (Canvas, Canvas) => true,
            (Caption, Caption) => true,
            (Center, Center) => true,
            (Cite, Cite) => true,
            (Code, Code) => true,
            (Col, Col) => true,
            (Colgroup, Colgroup) => true,
            (Data, Data) => true,
            (Datalist, Datalist) => true,
            (Dd, Dd) => true,
            (Del, Del) => true,
            (Details, Details) => true,
            (Dfn, Dfn) => true,
            (Dialog, Dialog) => true,
            (Dir, Dir) => true,
            (Div, Div) => true,
            (Dl, Dl) => true,
            (Dt, Dt) => true,
            (Em, Em) => true,
            (Embed, Embed) => true,
            (Fieldset, Fieldset) => true,
            (Figcaption, Figcaption) => true,
            (Figure, Figure) => true,
            (Font, Font) => true,
            (Footer, Footer) => true,
            (Form, Form) => true,
            (Frame, Frame) => true,
            (Frameset, Frameset) => true,
            (H1, H1) => true,
            (H2, H2) => true,
            (H3, H3) => true,
            (H4, H4) => true,
            (H5, H5) => true,
            (H6, H6) => true,
            (Head, Head) => true,
            (Header, Header) => true,
            (Hgroup, Hgroup) => true,
            (Hr, Hr) => true,
            (Html, Html) => true,
            (I, I) => true,
            (Iframe, Iframe) => true,
            (Img, Img) => true,
            (Input, Input) => true,
            (Ins, Ins) => true,
            (Kbd, Kbd) => true,
            (Keygen, Keygen) => true,
            (Label, Label) => true,
            (Legend, Legend) => true,
            (Li, Li) => true,
            (Link, Link) => true,
            (Listing, Listing) => true,
            (Main, Main) => true,
            (Map, Map) => true,
            (Mark, Mark) => true,
            (Marquee, Marquee) => true,
            (Math, Math) => true,
            (Menu, Menu) => true,
            (Menuitem, Menuitem) => true,
            (Meta, Meta) => true,
            (Meter, Meter) => true,
            (Nav, Nav) => true,
            (Nobr, Nobr) => true,
            (Noembed, Noembed) => true,
            (Noframes, Noframes) => true,
            (Noscript, Noscript) => true,
            (Object, Object) => true,
            (Ol, Ol) => true,
            (Optgroup, Optgroup) => true,
            (Option, Option) => true,
            (Output, Output) => true,
            (P, P) => true,
            (Param, Param) => true,
            (Picture, Picture) => true,
            (Plaintext, Plaintext) => true,
            (Pre, Pre) => true,
            (Progress, Progress) => true,
            (Q, Q) => true,
            (Rb, Rb) => true,
            (Rp, Rp) => true,
            (Rt, Rt) => true,
            (Rtc, Rtc) => true,
            (Ruby, Ruby) => true,
            (S, S) => true,
            (Samp, Samp) => true,
            (Script, Script) => true,
            (Section, Section) => true,
            (Select, Select) => true,
            (Slot, Slot) => true,
            (Small, Small) => true,
            (Source, Source) => true,
            (Span, Span) => true,
            (Strike, Strike) => true,
            (Strong, Strong) => true,
            (Style, Style) => true,
            (Sub, Sub) => true,
            (Summary, Summary) => true,
            (Sup, Sup) => true,
            (Svg, Svg) => true,
            (Table, Table) => true,
            (Tbody, Tbody) => true,
            (Td, Td) => true,
            (Template, Template) => true,
            (Textarea, Textarea) => true,
            (Tfoot, Tfoot) => true,
            (Th, Th) => true,
            (Thead, Thead) => true,
            (Time, Time) => true,
            (Title, Title) => true,
            (Tr, Tr) => true,
            (Track, Track) => true,
            (Tt, Tt) => true,
            (U, U) => true,
            (Ul, Ul) => true,
            (Var, Var) => true,
            (Video, Video) => true,
            (Wbr, Wbr) => true,
            (Xmp, Xmp) => true,

            (Mglyph, Mglyph) => true,
            (Malignmark, Malignmark) => true,
            (AnnotationXml, AnnotationXml) => true,
            (Mi, Mi) => true,
            (Mo, Mo) => true,
            (Mn, Mn) => true,
            (Ms, Ms) => true,
            (Mtext, Mtext) => true,
            (ForeignObject, ForeignObject) => true,
            (Desc, Desc) => true,

            (Other(left), Other(right)) => left.eq_ignore_ascii_case(right),
            (Other(left_str), right) => {
                if let Ok(left) = left_str.parse::<TagName>() {
                    if left.is_other() {
                        // We have to avoid recursion here
                        false
                    } else {
                        &left == right
                    }
                } else {
                    false
                }
            }
            (left, Other(right_str)) => {
                if let Ok(right) = right_str.parse::<TagName>() {
                    if right.is_other() {
                        // We have to avoid recursion here
                        false
                    } else {
                        left == &right
                    }
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}

impl std::hash::Hash for TagName {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.to_string().hash(state)
    }
}

impl std::fmt::Display for TagName {
    #[allow(clippy::too_many_lines, clippy::enum_glob_use)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use TagName::*;

        match self {
            A => write!(f, "a"),
            Abbr => write!(f, "abbr"),
            Address => write!(f, "address"),
            Applet => write!(f, "applet"),
            Area => write!(f, "area"),
            Article => write!(f, "article"),
            Aside => write!(f, "aside"),
            Audio => write!(f, "audio"),
            B => write!(f, "b"),
            Base => write!(f, "base"),
            Basefont => write!(f, "basefont"),
            Bdi => write!(f, "bdi"),
            Bdo => write!(f, "bdo"),
            Bgsound => write!(f, "bgsound"),
            Big => write!(f, "big"),
            Blink => write!(f, "blink"),
            Blockquote => write!(f, "blockquote"),
            Body => write!(f, "body"),
            Br => write!(f, "br"),
            Button => write!(f, "button"),
            Canvas => write!(f, "canvas"),
            Caption => write!(f, "caption"),
            Center => write!(f, "center"),
            Cite => write!(f, "cite"),
            Code => write!(f, "code"),
            Col => write!(f, "col"),
            Colgroup => write!(f, "colgroup"),
            Data => write!(f, "data"),
            Datalist => write!(f, "datalist"),
            Dd => write!(f, "dd"),
            Del => write!(f, "del"),
            Details => write!(f, "details"),
            Dfn => write!(f, "dfn"),
            Dialog => write!(f, "dialog"),
            Dir => write!(f, "dir"),
            Div => write!(f, "div"),
            Dl => write!(f, "dl"),
            Dt => write!(f, "dt"),
            Em => write!(f, "em"),
            Embed => write!(f, "embed"),
            Fieldset => write!(f, "fieldset"),
            Figcaption => write!(f, "figcaption"),
            Figure => write!(f, "figure"),
            Font => write!(f, "font"),
            Footer => write!(f, "footer"),
            Form => write!(f, "form"),
            Frame => write!(f, "frame"),
            Frameset => write!(f, "frameset"),
            H1 => write!(f, "h1"),
            H2 => write!(f, "h2"),
            H3 => write!(f, "h3"),
            H4 => write!(f, "h4"),
            H5 => write!(f, "h5"),
            H6 => write!(f, "h6"),
            Head => write!(f, "head"),
            Header => write!(f, "header"),
            Hgroup => write!(f, "hgroup"),
            Hr => write!(f, "hr"),
            Html => write!(f, "html"),
            I => write!(f, "i"),
            Iframe => write!(f, "iframe"),
            Img => write!(f, "img"),
            Input => write!(f, "input"),
            Ins => write!(f, "ins"),
            Kbd => write!(f, "kbd"),
            Keygen => write!(f, "keygen"),
            Label => write!(f, "label"),
            Legend => write!(f, "legend"),
            Li => write!(f, "li"),
            Link => write!(f, "link"),
            Listing => write!(f, "listing"),
            Main => write!(f, "main"),
            Map => write!(f, "map"),
            Mark => write!(f, "mark"),
            Marquee => write!(f, "marquee"),
            Math => write!(f, "math"),
            Menu => write!(f, "menu"),
            Menuitem => write!(f, "menuitem"),
            Meta => write!(f, "meta"),
            Meter => write!(f, "meter"),
            Nav => write!(f, "nav"),
            Nobr => write!(f, "nobr"),
            Noembed => write!(f, "noembed"),
            Noframes => write!(f, "noframes"),
            Noscript => write!(f, "noscript"),
            Object => write!(f, "object"),
            Ol => write!(f, "ol"),
            Optgroup => write!(f, "optgroup"),
            Option => write!(f, "option"),
            Output => write!(f, "output"),
            P => write!(f, "p"),
            Param => write!(f, "param"),
            Picture => write!(f, "picture"),
            Plaintext => write!(f, "plaintext"),
            Pre => write!(f, "pre"),
            Progress => write!(f, "progress"),
            Q => write!(f, "q"),
            Rb => write!(f, "rb"),
            Rp => write!(f, "rp"),
            Rt => write!(f, "rt"),
            Rtc => write!(f, "rtc"),
            Ruby => write!(f, "ruby"),
            S => write!(f, "s"),
            Samp => write!(f, "samp"),
            Script => write!(f, "script"),
            Section => write!(f, "section"),
            Select => write!(f, "select"),
            Slot => write!(f, "slot"),
            Small => write!(f, "small"),
            Source => write!(f, "source"),
            Span => write!(f, "span"),
            Strike => write!(f, "strike"),
            Strong => write!(f, "strong"),
            Style => write!(f, "style"),
            Sub => write!(f, "sub"),
            Summary => write!(f, "summary"),
            Sup => write!(f, "sup"),
            Svg => write!(f, "svg"),
            Table => write!(f, "table"),
            Tbody => write!(f, "tbody"),
            Td => write!(f, "td"),
            Template => write!(f, "template"),
            Textarea => write!(f, "textarea"),
            Tfoot => write!(f, "tfoot"),
            Th => write!(f, "th"),
            Thead => write!(f, "thead"),
            Time => write!(f, "time"),
            Title => write!(f, "title"),
            Tr => write!(f, "tr"),
            Track => write!(f, "track"),
            Tt => write!(f, "tt"),
            U => write!(f, "u"),
            Ul => write!(f, "ul"),
            Var => write!(f, "var"),
            Video => write!(f, "video"),
            Wbr => write!(f, "wbr"),
            Xmp => write!(f, "xmp"),

            Mglyph => write!(f, "mglyph"),
            Malignmark => write!(f, "malignmark"),
            AnnotationXml => write!(f, "annotationxml"),
            Mi => write!(f, "mi"),
            Mo => write!(f, "mo"),
            Mn => write!(f, "mn"),
            Ms => write!(f, "ms"),
            Mtext => write!(f, "mtext"),
            ForeignObject => write!(f, "foreignobject"),
            Desc => write!(f, "desc"),

            Other(s) => write!(f, "{}", s.to_lowercase()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn eq_self_a() {
        assert_eq!(TagName::A, TagName::A)
    }

    #[test]
    fn eq_self_string_a() {
        assert_eq!(TagName::A, TagName::Other("A".to_string()));
        assert_eq!(TagName::A, TagName::Other("a".to_string()));
        assert_eq!(TagName::Other("a".to_string()), TagName::A);
    }

    #[test]
    fn eq_self_legend() {
        assert_eq!(TagName::Legend, TagName::Legend)
    }

    #[test]
    fn eq_self_string_legend() {
        assert_eq!(TagName::Legend, TagName::Other("Legend".to_string()));
        assert_eq!(TagName::Legend, TagName::Other("legend".to_string()));
    }

    #[test]
    fn eq_not_self() {
        assert_ne!(TagName::Legend, TagName::Canvas)
    }

    #[test]
    fn eq_not_self_string_legend() {
        assert_ne!(TagName::Legend, TagName::Other("Canvas".to_string()));
        assert_ne!(TagName::Legend, TagName::Other("canvas".to_string()));
    }

    #[test]
    fn eq_not_self_string_unknown() {
        assert_ne!(TagName::Legend, TagName::Other("foo".to_string()));
    }

    #[test]
    fn display_a() {
        assert_eq!(TagName::Canvas.to_string(), "canvas".to_string())
    }
}
