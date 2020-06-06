#![allow(unused_imports)]

use log::trace;

use super::{states::*, TransitionResult};
use crate::dom;
use crate::tokenizer::Token;

/*
 * Transition Impls
 */

// TODO: unwraps

impl Initial {
    pub(super) fn on_token(self, t: Token) -> TransitionResult {
        match t {
            Token::Doctype(_d) => {
                /*
                let is_legacy_compat = d.system_identifier.is_some()
                    && d.system_identifier.unwrap() == "about:legacy-compat";
                if d.name != Some("html".to_string())
                    || d.public_identifier.is_some()
                    || is_legacy_compat
                {
                    panic!("Initial::on_token: Parse Error")
                }

                // Append a DocumentType node to the Document node,
                // with the name attribute set to the name given in the DOCTYPE token, or the empty string if the name was missing;
                // the publicId attribute set to the public identifier given in the DOCTYPE token, or the empty string if the public identifier was missing;
                // the systemId attribute set to the system identifier given in the DOCTYPE token, or the empty string if the system identifier was missing;
                // and the other attributes specific to DocumentType objects set to null and empty lists as appropriate.
                // Associate the DocumentType node with the Document object so that it is returned as the value of the doctype attribute of the Document object.
                let name = d.name.unwrap_or("".to_string());
                let publicId = d.public_identifier.unwrap_or("".to_string());
                let systemId = d.system_identifier.unwrap_or("".to_string());

                let document_type = dom::DocumentType::new(name, publicId, systemId);
                */
                todo!("Initial::on_token(DOCTYPE)")
            }
            _ => todo!("Initial::on_token(_)"),
        }
    }
}
