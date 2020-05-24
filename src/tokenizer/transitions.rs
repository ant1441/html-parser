use log::debug;

use super::*;

transitions!(StateMachine,
  [
    (Data, Character) => [CharacterReference, TagOpen, Error],
    // (RcData, Character) => [CharacterReference, RcDataLessThanSign],
    // (RawText, Character) => RawTextLessThanSign

    (TagOpen, Character) => [MarkupDeclarationOpen, EndTagOpen, TagName, BogusComment, Data],
    (EndTagOpen, Character) => [RcDataEndTagName, RcData],
    (TagName, Character) => [BeforeAttributeName, SelfClosingStartTag, Data, TagName],

    (RcDataEndTagName, Character) => [BeforeAttributeName, SelfClosingStartTag, Data, TagName, RcData],

    (MarkupDeclarationOpen, NextFewCharacters) => [CommentStart, Doctype, CdataSection, BogusComment],

    (Doctype, Character) => [BeforeDoctypeName, Error],
    (BeforeDoctypeName, Character) => [DoctypeName, Data, Error],
    (DoctypeName, Character) => [AfterDoctypeName, Data, DoctypeName, Error],

    (CharacterReference, Character) => [NumericCharacterReference, Error],
    (NamedCharacterReference, Character) => [NamedCharacterReference, AmbiguousAmpersand],


    (NumericCharacterReferenceEnd, Character) => [/* TODO */Data, Error]
  ]
);

impl StateMachine {
    pub fn new() -> Self {
        StateMachine::data()
    }
}

impl Default for StateMachine {
    fn default() -> Self {
        StateMachine::data()
    }
}

/*
 * Transition Impls
 */

impl Data {
    pub fn on_character(self, c: Character) -> StateMachine {
        match c {
            Character::Char('&') => StateMachine::CharacterReference(CharacterReference {
                return_state: Box::new(StateMachine::Data(self)),
            }),
            Character::Char('<') => StateMachine::tag_open(),
            // Emit the current input character as a character token.
            Character::LineFeed => {
                let token = Token::Character('\n');
                emit(token);
                StateMachine::data()
            }
            Character::Char(c) => {
                let token = Token::Character(c);
                emit(token);
                StateMachine::data()
            }
            //     This is an unexpected-null-character parse error. Emit the current input character as a character token.
            Character::Null => todo!("Data::NULL"),
            // Emit an end-of-file token.
            Character::EOF => {
                emit(Token::EOF);
                StateMachine::term()
            }
        }
    }
}

///

impl TagOpen {
    pub fn on_character(self, c: Character) -> StateMachine {
        match c {
            Character::Char('!') => StateMachine::markup_declaration_open(),
            Character::Char('/') => StateMachine::end_tag_open(),
            Character::Char(a) if a.is_alphabetic() => {
                let token = token::StartTag {
                    ..Default::default()
                };
                let reconsume_state = StateMachine::tag_name(token.into());
                debug!("Reconsume on State: {:?}", reconsume_state);
                reconsume_state.on_character(c)
            }
            Character::Char('?') => todo!("TagOpen:?"),
            Character::EOF => todo!("TagOpen::EOF"),
            _ => todo!("TagOpen::_"),
        }
    }
}

impl EndTagOpen {
    pub fn on_character(self, c: Character) -> StateMachine {
        match c {
            Character::Char(a) if a.is_alphabetic() => {
                let token = token::EndTag {
                    ..Default::default()
                };
                let reconsume_state = StateMachine::tag_name(token.into());
                debug!("Reconsume on State: {:?}", reconsume_state);
                reconsume_state.on_character(c)
            }
            Character::Char('>') => {
                todo!("missing-end-tag-name parse error");
                // StateMachine::data()
            }
            Character::EOF => {
                todo!("eof-before-tag-name parse error");
                // emit(Token::Character('\u{003C}'));
                // emit(Token::Character('\u{002F}'));
                // emit(Token::EOF);
            }
            _ => {
                todo!("invalid-first-character-of-tag-name parse error");
                // let reconsume_state = StateMachine::bogus_comment(String::new().into());
                // debug!("Reconsume on State: {:?}", reconsume_state);
                // reconsume_state.on_character(c)
            }
        }
    }
}

impl TagName {
    pub fn on_character(mut self, c: Character) -> StateMachine {
        match self.token {
            Token::StartTag(_) | Token::EndTag(_) => (),
            _ => unreachable!(),
        };
        match c {
            Character::Char('\t')
            | Character::LineFeed
            | Character::Char('\n')
            | Character::Char(' ') => StateMachine::before_attribute_name(),
            Character::Char('/') => StateMachine::self_closing_start_tag(),
            Character::Char('>') => {
                emit(self.token);
                StateMachine::data()
            }
            Character::Char(c) if c.is_ascii_uppercase() => {
                self.token.push(c.to_lowercase().next().unwrap());
                StateMachine::tag_name(self.token)
            }
            Character::Null => {
                todo!("TagName::on_character - unexpected-null-character parse error");
                // token.push('\u{FFFD}');
                // StateMachine::tag_name(token.into())
            }
            Character::EOF => todo!("TagName::on_character(EOF)"),
            Character::Char(c) => {
                self.token.push(c);
                StateMachine::tag_name(self.token)
            }
        }
    }
}

impl RcDataEndTagName {
    pub fn on_character(mut self, c: Character) -> StateMachine {
        match c {
            Character::Char('\t')
            | Character::LineFeed
            | Character::Char('\n')
            | Character::Char(' ')
                if self.token.is_appropriate_end_tag() =>
            {
                StateMachine::before_attribute_name()
            }
            Character::Char('/') if self.token.is_appropriate_end_tag() => {
                StateMachine::self_closing_start_tag()
            }
            Character::Char('>') if self.token.is_appropriate_end_tag() => {
                emit(self.token);
                StateMachine::data()
            }
            Character::Char(c) if c.is_ascii_uppercase() => {
                self.token.push(c.to_lowercase().next().unwrap());
                self.tmp.push(c);
                StateMachine::rc_data_end_tag_name(self.token, self.tmp)
            }
            Character::Char(c) if c.is_ascii_lowercase() => {
                self.token.push(c);
                self.tmp.push(c);
                StateMachine::rc_data_end_tag_name(self.token, self.tmp)
            }
            _ => {
                emit(Token::Character('\u{003C}'));
                emit(Token::Character('\u{002F}'));
                for c in self.tmp.chars() {
                    emit(Token::Character(c));
                }
                let reconsume_state = StateMachine::rc_data();
                debug!("Reconsume on State: {:?}", reconsume_state);
                reconsume_state.on_character(c)
            }
        }
    }
}

impl MarkupDeclarationOpen {
    pub fn on_next_few_characters(self, next: NextFewCharacters) -> StateMachine {
        if next.as_ref().is_none() {
            todo!("MarkupDeclarationOpen::on_next_few_characters(*)")
        } else {
            match next.as_ref().as_ref().unwrap().as_str() {
                "DOCTYPE" => StateMachine::doctype(),
                "--" => todo!("MarkupDeclarationOpen::on_next_few_characters(--)"),
                "[CDATA[" => todo!("MarkupDeclarationOpen::on_next_few_characters([CDATA[)"),
                _ => unreachable!(),
            }
        }
    }
}

impl Doctype {
    pub fn on_character(self, c: Character) -> StateMachine {
        match c {
            Character::Char('\t')
            | Character::LineFeed
            | Character::Char('\n')
            | Character::Char(' ') => StateMachine::before_doctype_name(),
            Character::Char('>') => {
                let reconsume_state = StateMachine::before_doctype_name();
                debug!("Reconsume on State: {:?}", reconsume_state);
                reconsume_state.on_character(c)
            }
            Character::EOF => todo!("Doctype::on_character(EOF)"),
            _ => todo!("Doctype::on_character(*)"),
        }
    }
}

impl BeforeDoctypeName {
    pub fn on_character(self, c: Character) -> StateMachine {
        match c {
            Character::Char('\t')
            | Character::LineFeed
            | Character::Char('\n')
            | Character::Char(' ') => StateMachine::before_doctype_name(),
            Character::Char(c) if c.is_ascii_uppercase() => {
                todo!("BeforeDoctypeName::on_character(UPPER)")
            }
            Character::Null => todo!("BeforeDoctypeName::on_character(NULL)"),
            Character::Char('>') => todo!("BeforeDoctypeName::on_character(>)"),
            Character::EOF => todo!("BeforeDoctypeName::on_character(EOF)"),
            Character::Char(c) => {
                let token = token::Doctype {
                    name: Some(c.to_string()),
                    ..Default::default()
                };
                StateMachine::doctype_name(token.into())
            }
        }
    }
}

impl DoctypeName {
    pub fn on_character(mut self, c: Character) -> StateMachine {
        match c {
            Character::Char('\t')
            | Character::LineFeed
            | Character::Char('\n')
            | Character::Char(' ') => StateMachine::after_doctype_name(),
            Character::Char('>') => {
                emit(self.token);
                StateMachine::data()
            }
            Character::Char(c) if c.is_ascii_uppercase() => {
                self.token.push(c.to_lowercase().next().unwrap());
                StateMachine::doctype_name(self.token)
            }
            Character::Null => todo!("DoctypeName::on_character(NULL)"),
            Character::EOF => todo!("DoctypeName::on_character(EOF)"),
            Character::Char(c) => {
                self.token.push(c);
                StateMachine::doctype_name(self.token)
            }
        }
    }
}

impl CharacterReference {
    pub fn on_character(self, c: Character) -> StateMachine {
        match c {
            Character::Char(a) if a.is_alphanumeric() => {
                // Reconsume
                let reconsume_state = NamedCharacterReference {};
                debug!("Reconsume on State: {:?}", reconsume_state);
                reconsume_state.on_character(c)
            },
            Character::Char('#') => todo!("CharacterReference::#"),
            _ => todo!("CharacterReference::AnythingElse - Flush code points consumed as a character reference. Reconsume in the return state."),
        }
    }
}

impl NamedCharacterReference {
    pub fn on_character(self, _: Character) -> StateMachine {
        todo!("NamedCharacterReference::on_character")
    }
}

impl NumericCharacterReferenceEnd {
    pub fn on_character(self, _: Character) -> StateMachine {
        todo!("NumericCharacterReferenceEnd::on_character")
    }
}
