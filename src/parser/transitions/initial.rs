use crate::{
    dom,
    parser::{states::*, Parser, TransitionResult},
    tokenizer::Token,
};
use std::io;

use super::parse_error;

impl Initial {
    pub(in crate::parser) fn on_token<R>(
        self,
        parser: &mut Parser<R>,
        t: &Token,
    ) -> TransitionResult
    where
        R: io::Read + io::Seek,
    {
        transition(States::from(self), parser, t)
    }
}

pub(super) fn transition<R>(
    current_state: States,
    parser: &mut Parser<R>,
    t: &Token,
) -> TransitionResult
where
    R: io::Read + io::Seek,
{
    match t {
        Token::Character('\t') | Token::Character('\n') | Token::Character(' ') => {
            current_state.into_transition_result()
        }
        Token::Comment(comment) => {
            let node = dom::Comment::new(comment.clone());
            parser.document.push(node);
            current_state.into_transition_result()
        }
        Token::Doctype(d) => {
            let is_force_quirks = d.is_force_quirks();
            let public_id_present = d.public_identifier.is_some();
            let system_id_present = d.system_identifier.is_some();
            let name = d.name.clone().unwrap_or_else(|| "".to_string());
            let public_id = d
                .public_identifier
                .clone()
                .unwrap_or_else(|| "".to_string());
            let system_id = d
                .system_identifier
                .clone()
                .unwrap_or_else(|| "".to_string());

            if name != "html"
                || public_id_present
                || system_id_present && system_id == "about:legacy-compat"
            {
                parse_error("Initial::on_token")
            }

            // Append a DocumentType node to the Document node,
            // with the name attribute set to the name given in the DOCTYPE token, or the empty string if the name was missing;
            // the publicId attribute set to the public identifier given in the DOCTYPE token, or the empty string if the public identifier was missing;
            // the systemId attribute set to the system identifier given in the DOCTYPE token, or the empty string if the system identifier was missing;
            // and the other attributes specific to DocumentType objects set to null and empty lists as appropriate.
            // Associate the DocumentType node with the Document object so that it is returned as the value of the doctype attribute of the Document object.

            if super::force_quirks_check::quirks_check(
                &name,
                &public_id,
                &system_id,
                is_force_quirks,
                system_id_present,
            ) {
                parser.document.set_mode("quirks");
            } else if super::force_quirks_check::limited_quirks_check(&public_id, system_id_present)
            {
                parser.document.set_mode("limited-quirks");
            }

            let document_type = dom::DocumentType::new(name, public_id, system_id);
            parser.document.add_document_type(document_type);

            States::before_html().into_transition_result()
        }
        _ => {
            // If the document is not an iframe srcdoc document, then this is a parse error; set the Document to quirks mode.
            let mut ret = States::before_html().into_transition_result();
            ret.set_reprocess();
            ret
        }
    }
}
