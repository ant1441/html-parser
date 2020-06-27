use log::{trace, warn};

use crate::{
    dom::{self, Namespace},
    parser::{self, states::*, FramesetOkFlag, Parser, TransitionResult},
    tokenizer::{TagName, Token},
};
use std::io;

use super::parse_error;

impl Text {
    pub(in crate::parser) fn on_token<R>(
        self,
        parser: &mut Parser<R>,
        t: &Token,
    ) -> TransitionResult
    where
        R: io::Read + io::Seek,
    {
    match t {
        Token::Character('\0') => unreachable!(),
        Token::Character(ch) => {
            parser.insert_character(ch.to_string());
            States::from(self).into_transition_result()
        }
        Token::EndTag(tag) if tag.name == TagName::Script => {
            todo!("Text::on_token(</script>)")
        }
        Token::EndTag(_) => {
            parser.open_elements.pop();
            self.original_insertion_mode.into_transition_result()
        }
        _ => unreachable!("Parser - Text State"),
    }
}
}
