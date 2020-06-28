use std::io;

use crate::{
    parser::{states::*, Parser, TransitionResult},
    tokenizer::{TagName, Token},
};

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
            let _ = parser.open_elements.pop();
            self.original_insertion_mode.into_transition_result()
        }
        _ => unreachable!("Parser - Text State"),
    }
}
}
