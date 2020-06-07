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
            Token::Character('\t') | Token::Character('\n') | Token::Character(' ') => todo!("do nothing"),
            Token::Doctype(d) => {
                let is_force_quirks = d.is_force_quirks();
                let public_id_present = d.public_identifier.is_some();
                let system_id_present = d.system_identifier.is_some();
                let name = d.name.unwrap_or_else(|| "".to_string());
                let public_id = d.public_identifier.unwrap_or_else(|| "".to_string());
                let system_id = d.system_identifier.unwrap_or_else(|| "".to_string());

                if name != "html"
                    || public_id_present
                    || system_id_present && system_id == "about:legacy-compat"
                {
                    panic!("Initial::on_token: Parse Error")
                }

                // Append a DocumentType node to the Document node,
                // with the name attribute set to the name given in the DOCTYPE token, or the empty string if the name was missing;
                // the publicId attribute set to the public identifier given in the DOCTYPE token, or the empty string if the public identifier was missing;
                // the systemId attribute set to the system identifier given in the DOCTYPE token, or the empty string if the system identifier was missing;
                // and the other attributes specific to DocumentType objects set to null and empty lists as appropriate.
                // Associate the DocumentType node with the Document object so that it is returned as the value of the doctype attribute of the Document object.

                // TODO
                let mut document: dom::Document = Default::default();

                if super::force_quirks_check::force_quirks_check(
                    &name,
                    &public_id,
                    &system_id,
                    is_force_quirks,
                    system_id_present,
                ) {
                    document.set_mode("quirks");
                }
                let document_type = dom::DocumentType::new(name, public_id, system_id);
                document.add_document_type(document_type);

                todo!("Initial::on_token(DOCTYPE)")
            }
            _ => todo!("Initial::on_token(_)"),
        }
    }
}
