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

    (BeforeAttributeName, Character) => [BeforeAttributeName, AfterAttributeName, AttributeName],
    (AttributeName, Character) => [AfterAttributeName, BeforeAttributeValue, AttributeName],
    // (AfterAttributeName, Character) => [],
    (BeforeAttributeValue, Character) => [AfterAttributeName, BeforeAttributeValue, AttributeName],
    (AttributeValueDoubleQuoted, Character) => [AfterAttributeValueQuoted, CharacterReference, AttributeValueDoubleQuoted],
    (AttributeValueSingleQuoted, Character) => [AfterAttributeValueQuoted, CharacterReference, AttributeValueSingleQuoted],
    (AttributeValueUnquoted, Character) => [BeforeAttributeName, CharacterReference, AttributeValueUnquoted],
    (AfterAttributeValueQuoted, Character) => [BeforeAttributeName, SelfClosingStartTag, Data, BeforeAttributeName],

    (MarkupDeclarationOpen, NextFewCharacters) => [CommentStart, Doctype, CdataSection, BogusComment],
    (CommentStart, Character) => [CommentStartDash, Data, Comment],
    // (CommentStartDash, Character) => [],
    (Comment, Character) => [CommentLessThanSign, CommentEndDash, Comment],

    (CommentEndDash, Character) => [CommentEnd, Comment],
    (CommentEnd, Character) => [Data, CommentEndBang, CommentEnd, Comment],

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
            Character::Char('&') => {
                StateMachine::character_reference(Box::new(StateMachine::Data(self)))
            }
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
            Character::Null => {
                parse_error("unexpected-null-character parse error");
                // Emit the current input character as a character token.
                todo!("Data::on_character(NULL) the current character?")
            }
            // Emit an end-of-file token.
            Character::Eof => {
                emit(Token::Eof);
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
            Character::Eof => todo!("TagOpen::EOF"),
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
                parse_error("missing-end-tag-name parse error");
                StateMachine::data()
            }
            Character::Eof => {
                parse_error("eof-before-tag-name parse error");
                emit(Token::Character('\u{003C}'));
                emit(Token::Character('\u{002F}'));
                emit(Token::Eof);
                StateMachine::term()
            }
            _ => {
                parse_error("invalid-first-character-of-tag-name parse error");
                let reconsume_state = StateMachine::bogus_comment(String::new().into());
                debug!("Reconsume on State: {:?}", reconsume_state);
                reconsume_state.on_character(c)
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
            | Character::Char(' ') => StateMachine::before_attribute_name(self.token),
            Character::Char('/') => StateMachine::self_closing_start_tag(self.token),
            Character::Char('>') => {
                emit(self.token);
                StateMachine::data()
            }
            Character::Char(c) if c.is_ascii_uppercase() => {
                self.token.push(c.to_lowercase().next().unwrap());
                StateMachine::tag_name(self.token)
            }
            Character::Null => {
                parse_error("unexpected-null-character parse error");
                self.token.push('\u{FFFD}');
                StateMachine::tag_name(self.token)
            }
            Character::Eof => {
                parse_error("eof-in-tag parse error");
                emit(Token::Eof);
                StateMachine::term()
            }
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
                StateMachine::before_attribute_name(self.token)
            }
            Character::Char('/') if self.token.is_appropriate_end_tag() => {
                StateMachine::self_closing_start_tag(self.token)
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

impl BeforeAttributeName {
    pub fn on_character(mut self, c: Character) -> StateMachine {
        match c {
            Character::Char('\t')
            | Character::LineFeed
            | Character::Char('\n')
            | Character::Char(' ') => StateMachine::before_attribute_name(self.token),
            Character::Char('/') | Character::Char('>') | Character::Eof => {
                let reconsume_state = StateMachine::after_attribute_name(self.token);
                debug!("Reconsume on State: {:?}", reconsume_state);
                reconsume_state.on_character(c)
            }
            Character::Char('=') => {
                parse_error("unexpected-equals-sign-before-attribute-name parse error");
                self.token.add_attribute("=".to_string(), String::new());
                StateMachine::attribute_name(self.token)
            }
            _ => {
                self.token.add_attribute(String::new(), String::new());
                let reconsume_state = StateMachine::attribute_name(self.token);
                debug!("Reconsume on State: {:?}", reconsume_state);
                reconsume_state.on_character(c)
            }
        }
    }
}

impl AttributeName {
    pub fn on_character(mut self, c: Character) -> StateMachine {
        match c {
            Character::Char('\t')
            | Character::LineFeed
            | Character::Char('\n')
            | Character::Char(' ')
            | Character::Char('/')
            | Character::Char('>')
            | Character::Eof => {
                self.check_duplicate_attribuite();
                let reconsume_state = StateMachine::after_attribute_name(self.token);
                debug!("Reconsume on State: {:?}", reconsume_state);
                reconsume_state.on_character(c)
            }
            Character::Char('=') => {
                self.check_duplicate_attribuite();
                StateMachine::before_attribute_value(self.token)
            }
            Character::Char(c) if c.is_ascii_uppercase() => {
                let attribute = self.token.current_attribute_mut().unwrap();
                attribute.push_name(c.to_lowercase().next().unwrap());
                StateMachine::attribute_name(self.token)
            }
            Character::Null => {
                parse_error("unexpected-null-character parse error");
                let attribute = self.token.current_attribute_mut().unwrap();
                attribute.push_name('\u{FFFD}');
                StateMachine::attribute_name(self.token)
            }
            Character::Char(c) => {
                if c == '"' || c == '\'' || c == '<' {
                    parse_error("unexpected-character-in-attribute-name")
                }
                let attribute = self.token.current_attribute_mut().unwrap();
                attribute.push_name(c);
                StateMachine::attribute_name(self.token)
            }
        }
    }

    fn check_duplicate_attribuite(&mut self) {
        // TODO
        /*
        if let Token::StartTag(tag) = self.token {
            let current_attribute = tag.current_attribute();
            let num_attributes = tag.attributes_iter().count();

            for (n, attribute) in tag.attributes_iter().enumerate() {
                if n == num_attributes {
                    break;
                }

                dbg!(attribute);
                /*
                if attribute.name == current_attribute.name {
                    current_attribute.set_duplicate();
                        break;
                }
                */

            }
                todo!()
        } else if let Token::EndTag(tag) = self.token {
            todo!()
        } else {
            panic!("Unexpected token in AttributeName::check_duplicate_attribuite");
        }
        */
    }
}

impl BeforeAttributeValue {
    pub fn on_character(self, c: Character) -> StateMachine {
        match c {
            Character::Char('\t')
            | Character::LineFeed
            | Character::Char('\n')
            | Character::Char(' ') => StateMachine::before_attribute_value(self.token),
            Character::Char('"') => StateMachine::attribute_value_double_quoted(self.token),
            Character::Char('\'') => StateMachine::attribute_value_single_quoted(self.token),
            Character::Char('>') => {
                parse_error("missing-attribute-value parse error");
                emit(self.token);
                StateMachine::data()
            }
            _ => {
                let reconsume_state = StateMachine::attribute_value_unquoted(self.token);
                debug!("Reconsume on State: {:?}", reconsume_state);
                reconsume_state.on_character(c)
            }
        }
    }
}

impl AttributeValueDoubleQuoted {
    pub fn on_character(mut self, c: Character) -> StateMachine {
        match c {
            Character::Char('"') => StateMachine::after_attribute_value_quoted(self.token),
            Character::Char('&') => StateMachine::character_reference(Box::new(
                StateMachine::AttributeValueDoubleQuoted(self),
            )),
            Character::Null => {
                parse_error("unexpected-null-character parse error");
                let attribute = self.token.current_attribute_mut().unwrap();
                attribute.push_value('\u{FFFD}');
                StateMachine::attribute_value_double_quoted(self.token)
            }
            Character::Eof => {
                parse_error("eof-in-tag parse error");
                emit(self.token);
                emit(Token::Eof);
                StateMachine::term()
            }
            Character::LineFeed => {
                let attribute = self.token.current_attribute_mut().unwrap();
                attribute.push_value('\n');
                StateMachine::attribute_value_double_quoted(self.token)
            }
            Character::Char(c) => {
                let attribute = self.token.current_attribute_mut().unwrap();
                attribute.push_value(c);
                StateMachine::attribute_value_double_quoted(self.token)
            }
        }
    }
}

impl AttributeValueSingleQuoted {
    pub fn on_character(mut self, c: Character) -> StateMachine {
        match c {
            Character::Char('\'') => StateMachine::after_attribute_value_quoted(self.token),
            Character::Char('&') => StateMachine::character_reference(Box::new(
                StateMachine::AttributeValueSingleQuoted(self),
            )),
            Character::Null => {
                parse_error("unexpected-null-character parse error");
                let attribute = self.token.current_attribute_mut().unwrap();
                attribute.push_value('\u{FFFD}');
                StateMachine::attribute_value_single_quoted(self.token)
            }
            Character::Eof => {
                parse_error("eof-in-tag parse error");
                emit(self.token);
                emit(Token::Eof);
                StateMachine::term()
            }
            Character::LineFeed => {
                let attribute = self.token.current_attribute_mut().unwrap();
                attribute.push_value('\n');
                StateMachine::attribute_value_single_quoted(self.token)
            }
            Character::Char(c) => {
                let attribute = self.token.current_attribute_mut().unwrap();
                attribute.push_value(c);
                StateMachine::attribute_value_single_quoted(self.token)
            }
        }
    }
}

impl AttributeValueUnquoted {
    pub fn on_character(mut self, c: Character) -> StateMachine {
        match c {
            Character::Char('\t')
            | Character::LineFeed
            | Character::Char('\n')
            | Character::Char(' ') => StateMachine::before_attribute_name(self.token),
            Character::Char('&') => StateMachine::character_reference(Box::new(
                StateMachine::AttributeValueUnquoted(self),
            )),
            Character::Char('>') => {
                emit(self.token);
                StateMachine::data()
            }
            Character::Null => {
                parse_error("unexpected-null-character parse error");
                let attribute = self.token.current_attribute_mut().unwrap();
                attribute.push_value('\u{FFFD}');
                StateMachine::attribute_value_unquoted(self.token)
            }

            Character::Eof => {
                parse_error("eof-in-tag parse error");
                emit(self.token);
                emit(Token::Eof);
                StateMachine::term()
            }
            Character::Char(c) => {
                if c == '"' || c == '\'' || c == '<' || c == '=' || c == '`' {
                    parse_error("unexpected-character-in-unquoted-attribute-value");
                }

                let attribute = self.token.current_attribute_mut().unwrap();
                attribute.push_value(c);
                StateMachine::attribute_value_unquoted(self.token)
            }
        }
    }
}

impl AfterAttributeValueQuoted {
    pub fn on_character(self, c: Character) -> StateMachine {
        match c {
            Character::Char('\t')
            | Character::LineFeed
            | Character::Char('\n')
            | Character::Char(' ') => StateMachine::before_attribute_name(self.token),
            Character::Char('/') => StateMachine::self_closing_start_tag(self.token),
            Character::Char('>') => {
                emit(self.token);
                StateMachine::data()
            }
            Character::Eof => {
                parse_error("eof-in-tag parse error");
                emit(self.token);
                emit(Token::Eof);
                StateMachine::term()
            }
            _ => {
                parse_error("missing-whitespace-between-attributes");

                let reconsume_state = StateMachine::before_attribute_name(self.token);
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
                "--" => StateMachine::comment_start(String::new().into()),
                "[CDATA[" => todo!("MarkupDeclarationOpen::on_next_few_characters([CDATA[)"),
                _ => unreachable!(),
            }
        }
    }
}

impl CommentStart {
    pub fn on_character(self, c: Character) -> StateMachine {
        match c {
            Character::Char('-') => StateMachine::comment_start_dash(self.token),
            Character::Char('>') => {
                parse_error("abrupt-closing-of-empty-comment parse error");
                emit(self.token);
                StateMachine::data()
            }
            _ => {
                let reconsume_state = StateMachine::comment(self.token);
                debug!("Reconsume on State: {:?}", reconsume_state);
                reconsume_state.on_character(c)
            }
        }
    }
}

impl CommentStartDash {
    pub fn on_character(mut self, c: Character) -> StateMachine {
        match c {
            Character::Char('-') => StateMachine::comment_end(self.token),
            Character::Char('>') => {
                parse_error("abrupt-closing-of-empty-comment parse error");
                emit(self.token);
                StateMachine::data()
            }
            Character::Eof => {
                parse_error("eof-in-comment parse error");
                emit(self.token);
                emit(Token::Eof);
                StateMachine::term()
            }
            _ => {
                self.token.push('-');

                let reconsume_state = StateMachine::comment(self.token);
                debug!("Reconsume on State: {:?}", reconsume_state);
                reconsume_state.on_character(c)
            }
        }
    }
}

impl Comment {
    pub fn on_character(mut self, c: Character) -> StateMachine {
        match c {
            Character::Char('<') => {
                self.token.push('<');
                StateMachine::comment_less_than_sign(self.token)
            }
            Character::Char('-') => StateMachine::comment_end_dash(self.token),
            Character::Null => {
                parse_error("unexpected-null-character parse error");
                self.token.push('\u{FFFD}');
                StateMachine::comment(self.token)
            }
            Character::Eof => {
                parse_error("eof-in-comment parse error");
                emit(self.token);
                emit(Token::Eof);
                StateMachine::term()
            }
            Character::LineFeed => {
                self.token.push('\n');
                StateMachine::comment(self.token)
            }
            Character::Char(c) => {
                self.token.push(c);
                StateMachine::comment(self.token)
            }
        }
    }
}

impl CommentEndDash {
    pub fn on_character(mut self, c: Character) -> StateMachine {
        match c {
            Character::Char('-') => StateMachine::comment_end(self.token),
            Character::Eof => {
                parse_error("eof-in-tag parse error");
                emit(self.token);
                emit(Token::Eof);
                StateMachine::term()
            }
            _ => {
                self.token.push('-');

                let reconsume_state = StateMachine::comment(self.token);
                debug!("Reconsume on State: {:?}", reconsume_state);
                reconsume_state.on_character(c)
            }
        }
    }
}

impl CommentEnd {
    pub fn on_character(mut self, c: Character) -> StateMachine {
        match c {
            Character::Char('>') => {
                emit(self.token);
                StateMachine::data()
            }
            Character::Char('!') => StateMachine::comment_end_bang(self.token),
            Character::Char('-') => {
                self.token.push('-');
                StateMachine::comment_end(self.token)
            }
            Character::Eof => {
                parse_error("eof-in-tag parse error");
                emit(self.token);
                emit(Token::Eof);
                StateMachine::term()
            }
            _ => {
                self.token.push('-');
                self.token.push('-');

                let reconsume_state = StateMachine::comment(self.token);
                debug!("Reconsume on State: {:?}", reconsume_state);
                reconsume_state.on_character(c)
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
            Character::Eof => todo!("Doctype::on_character(EOF)"),
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
            Character::Eof => todo!("BeforeDoctypeName::on_character(EOF)"),
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
            Character::Eof => todo!("DoctypeName::on_character(EOF)"),
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
