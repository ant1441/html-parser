use derive_more::From;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Eq, From, Hash, PartialEq, Serialize)]
pub enum Namespace {
    HTML,
    MathML,
    SVG,
    XLink,
    XML,
    XMLNS,
}

impl Namespace {
    pub fn from_url<T: AsRef<str>>(url: T) -> Result<Self, &'static str> {
        use Namespace::*;
        match url.as_ref() {
            "http://www.w3.org/1999/xhtml" => Ok(HTML),
            "http://www.w3.org/1998/Math/MathML" => Ok(MathML),
            "http://www.w3.org/2000/svg" => Ok(SVG),
            "http://www.w3.org/1999/xlink" => Ok(XLink),
            "http://www.w3.org/XML/1998/namespace" => Ok(XML),
            "http://www.w3.org/2000/xmlns/" => Ok(XMLNS),
            _ => Err("Unknown Namespace"),
        }
    }
}

impl Default for Namespace {
    fn default() -> Self {
        Namespace::HTML
    }
}
