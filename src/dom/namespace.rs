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
    /// Parse `Namespace` from a url
    /// All namespaces can be found in the [spec](https://infra.spec.whatwg.org/#namespaces)
    ///
    /// # Errors
    /// Fails if an unknown namespace is passed
    pub fn from_url<T: AsRef<str>>(url: T) -> Result<Self, &'static str> {
        match url.as_ref() {
            "http://www.w3.org/1999/xhtml" => Ok(Namespace::HTML),
            "http://www.w3.org/1998/Math/MathML" => Ok(Namespace::MathML),
            "http://www.w3.org/2000/svg" => Ok(Namespace::SVG),
            "http://www.w3.org/1999/xlink" => Ok(Namespace::XLink),
            "http://www.w3.org/XML/1998/namespace" => Ok(Namespace::XML),
            "http://www.w3.org/2000/xmlns/" => Ok(Namespace::XMLNS),
            _ => Err("Unknown Namespace"),
        }
    }
}

impl Default for Namespace {
    fn default() -> Self {
        Namespace::HTML
    }
}
