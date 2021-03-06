#![allow(clippy::wildcard_imports, clippy::unused_self)]

use log::trace;

use crate::tokenizer::{
    codepoint,
    errors::ParseError,
    get_entities,
    states::*,
    token::{Doctype as DoctypeToken, EndTag, ForceQuirksFlag, SelfClosingFlag, StartTag, Token},
    TransitionResult,
};

const U_AMPERSAND: char = '\u{0026}'; // '&'
const U_APOSTROPHE: char = '\u{0027}'; // '\''
const U_CHARACTER_TABULATION: char = '\u{0009}'; // '\t'
const U_EQUALS_SIGN: char = '\u{003D}'; // '='
const U_EXCLAMATION_MARK: char = '\u{0021}'; // '!'
const U_FORM_FEED: char = '\u{000C}'; // '\x0c'
const U_GRAVE_ACCENT: char = '\u{0060}'; // '`'
const U_GREATER_THAN_SIGN: char = '\u{003E}'; // '>'
const U_HYPHEN_MINUS: char = '\u{002D}'; // '-'
const U_LATIN_CAPITAL_LETTER_X: char = '\u{0058}'; // 'X'
const U_LATIN_SMALL_LETTER_X: char = '\u{0078}'; // 'x'
const U_LESS_THAN_SIGN: char = '\u{003C}'; // '<'
const U_LINE_FEED: char = '\u{000A}'; // '\n'
const U_NULL: char = '\u{0000}'; // '\0'
const U_NUMBER_SIGN: char = '\u{0023}'; // '#'
const U_QUESTION_MARK: char = '\u{003F}'; // '?'
const U_QUOTATION_MARK: char = '\u{0022}'; // '"'
const U_REPLACEMENT_CHARACTER: char = '\u{FFFD}'; // '�'
const U_SEMICOLON: char = '\u{003B}'; // ';'
const U_SOLIDUS: char = '\u{002F}'; // '/'
const U_SPACE: char = '\u{0020}'; // ' '

/*
 * Transition Impls
 */

// TODO: unwraps

impl Data {
    pub(super) fn on_character(self, c: Character) -> TransitionResult {
        match c {
            Character::Char(U_AMPERSAND) => {
                States::character_reference(self, String::new()).into_transition_result()
            }
            Character::Char(U_LESS_THAN_SIGN) => States::tag_open().into_transition_result(),
            Character::Char(U_NULL) => {
                let mut ret = States::from(self).into_transition_result();
                ret.push_parse_error(ParseError::UnexpectedNullCharacter);
                // Not sure if this should be NULL
                ret.push_emit(U_NULL);
                ret
            }
            Character::Eof => {
                let mut ret = States::term().into_transition_result();
                ret.push_emit(Token::Eof);
                ret
            }
            Character::Char(c) => {
                let mut ret = States::from(self).into_transition_result();
                ret.push_emit(c);
                ret
            }
        }
    }
}

impl RcData {
    pub(super) fn on_character(self, c: Character) -> TransitionResult {
        match c {
            Character::Char(U_AMPERSAND) => {
                States::character_reference(self, String::new()).into_transition_result()
            }
            Character::Char(U_LESS_THAN_SIGN) => {
                States::rc_data_less_than_sign(self.tmp).into_transition_result()
            }
            Character::Char(U_NULL) => {
                let mut ret = States::from(self).into_transition_result();
                ret.push_parse_error(ParseError::UnexpectedNullCharacter);
                ret.push_emit(U_REPLACEMENT_CHARACTER);
                ret
            }
            Character::Eof => {
                let mut ret = States::term().into_transition_result();
                ret.push_emit(Token::Eof);
                ret
            }
            Character::Char(c) => {
                let mut ret = States::from(self).into_transition_result();
                ret.push_emit(c);
                ret
            }
        }
    }
}

impl RawText {
    pub(super) fn on_character(self, c: Character) -> TransitionResult {
        match c {
            Character::Char(U_LESS_THAN_SIGN) => {
                States::raw_text_less_than_sign(self.tmp).into_transition_result()
            }
            Character::Char(U_NULL) => {
                let mut ret = States::from(self).into_transition_result();
                ret.push_parse_error(ParseError::UnexpectedNullCharacter);
                ret.push_emit(U_REPLACEMENT_CHARACTER);
                ret
            }
            Character::Eof => {
                let mut ret = States::term().into_transition_result();
                ret.push_emit(Token::Eof);
                ret
            }
            Character::Char(c) => {
                let mut ret = States::from(self).into_transition_result();
                ret.push_emit(c);
                ret
            }
        }
    }
}

impl ScriptData {
    pub(super) fn on_character(self, c: Character) -> TransitionResult {
        match c {
            Character::Char(U_LESS_THAN_SIGN) => {
                States::script_data_less_than_sign().into_transition_result()
            }
            Character::Char(U_NULL) => {
                let mut ret = States::from(self).into_transition_result();
                ret.push_parse_error(ParseError::UnexpectedNullCharacter);
                ret.push_emit(U_REPLACEMENT_CHARACTER);
                ret
            }
            Character::Eof => {
                let mut ret = States::term().into_transition_result();
                ret.push_emit(Token::Eof);
                ret
            }
            Character::Char(c) => {
                let mut ret = States::from(self).into_transition_result();
                ret.push_emit(c);
                ret
            }
        }
    }
}

impl PlainText {
    pub(super) fn on_character(self, c: Character) -> TransitionResult {
        match c {
            Character::Char(U_NULL) => {
                let mut ret = States::from(self).into_transition_result();
                ret.push_parse_error(ParseError::UnexpectedNullCharacter);
                ret.push_emit(U_REPLACEMENT_CHARACTER);
                ret
            }
            Character::Eof => {
                let mut ret = States::term().into_transition_result();
                ret.push_emit(Token::Eof);
                ret
            }
            Character::Char(c) => {
                let mut ret = States::from(self).into_transition_result();
                ret.push_emit(c);
                ret
            }
        }
    }
}

impl TagOpen {
    pub(super) fn on_character(self, c: Character) -> TransitionResult {
        match c {
            Character::Char(U_EXCLAMATION_MARK) => {
                States::markup_declaration_open().into_transition_result()
            }
            Character::Char(U_SOLIDUS) => States::end_tag_open().into_transition_result(),
            Character::Char(a) if a.is_alphabetic() => {
                let token = StartTag::default();
                let mut ret = States::tag_name(token).into_transition_result();
                ret.set_reconsume();
                ret
            }
            Character::Char(U_QUESTION_MARK) => {
                let mut ret = States::bogus_comment(String::new()).into_transition_result();
                ret.push_parse_error(ParseError::UnexpectedQuestionMarkInsteadOfTagName);
                ret.set_reconsume();
                ret
            }
            Character::Eof => {
                let mut ret = States::term().into_transition_result();
                ret.push_parse_error(ParseError::EofBeforeTagName);
                ret.push_emit(U_LESS_THAN_SIGN);
                ret.push_emit(Token::Eof);
                ret
            }
            _ => {
                let mut ret = States::data().into_transition_result();
                ret.push_emit(U_LESS_THAN_SIGN);
                ret.push_parse_error(ParseError::InvalidFirstCharacterOfTagName);
                ret.set_reconsume();
                ret
            }
        }
    }
}

impl EndTagOpen {
    pub(super) fn on_character(self, c: Character) -> TransitionResult {
        match c {
            Character::Char(a) if a.is_alphabetic() => {
                let token = EndTag::default();
                let mut ret = States::tag_name(token).into_transition_result();
                ret.set_reconsume();
                ret
            }
            Character::Char(U_GREATER_THAN_SIGN) => {
                let mut ret = States::data().into_transition_result();
                ret.push_parse_error(ParseError::MissingEndTagName);
                ret
            }
            Character::Eof => {
                let mut ret = States::term().into_transition_result();
                ret.push_parse_error(ParseError::EofBeforeTagName);
                ret.push_emit(U_LESS_THAN_SIGN);
                ret.push_emit(U_SOLIDUS);
                ret.push_emit(Token::Eof);
                ret
            }
            _ => {
                let mut ret = States::bogus_comment(String::new()).into_transition_result();
                ret.push_parse_error(ParseError::InvalidFirstCharacterOfTagName);
                ret.set_reconsume();
                ret
            }
        }
    }
}

impl TagName {
    pub(super) fn on_character(mut self, c: Character) -> TransitionResult {
        match self.token {
            Token::StartTag(_) | Token::EndTag(_) => (),
            _ => unreachable!("TagName state entered with a none Tag Token"),
        };
        match c {
            Character::Char(U_CHARACTER_TABULATION)
            | Character::Char(U_LINE_FEED)
            | Character::Char(U_FORM_FEED)
            | Character::Char(U_SPACE) => {
                States::before_attribute_name(self.token).into_transition_result()
            }
            Character::Char(U_SOLIDUS) => {
                States::self_closing_start_tag(self.token).into_transition_result()
            }
            Character::Char(U_GREATER_THAN_SIGN) => {
                let mut ret = States::data().into_transition_result();
                ret.push_emit(self.token);
                ret
            }
            Character::Char(c) if c.is_ascii_uppercase() => {
                self.token.push(c.to_lowercase().next().unwrap());

                States::from(self).into_transition_result()
            }
            Character::Char(U_NULL) => {
                self.token.push(U_REPLACEMENT_CHARACTER);

                let mut ret = States::from(self).into_transition_result();
                ret.push_parse_error(ParseError::UnexpectedNullCharacter);
                ret
            }
            Character::Eof => {
                let mut ret = States::term().into_transition_result();
                ret.push_parse_error(ParseError::EofInTag);
                ret.push_emit(Token::Eof);
                ret
            }
            Character::Char(c) => {
                self.token.push(c);

                States::from(self).into_transition_result()
            }
        }
    }
}

impl RcDataLessThanSign {
    pub(super) fn on_character(self, c: Character) -> TransitionResult {
        #[allow(clippy::single_match_else)]
        match c {
            Character::Char(U_SOLIDUS) => States::rc_data_end_tag_open(String::new()).into_transition_result(),
            _ => {
                let mut ret = States::rc_data(self.tmp).into_transition_result();
                ret.push_emit(U_LESS_THAN_SIGN);
                ret.set_reconsume();
                ret
            }
        }
    }
}

impl RcDataEndTagOpen {
    pub(super) fn on_character(self, c: Character) -> TransitionResult {
        match c {
            Character::Char(a) if a.is_alphabetic() => {
                let token = EndTag::default();
                let mut ret =
                    States::rc_data_end_tag_name(token, self.tmp).into_transition_result();
                ret.set_reconsume();
                ret
            }
            _ => {
                let mut ret = States::rc_data(self.tmp).into_transition_result();
                ret.push_emit(U_LESS_THAN_SIGN);
                ret.push_emit(U_SOLIDUS);
                ret.set_reconsume();
                ret
            }
        }
    }
}

impl RcDataEndTagName {
    pub(super) fn on_character_and_last_start_tag(
        mut self,
        c: CharacterAndLastStartTag,
    ) -> TransitionResult {
        let (c, last_start_tag_emitted) = c.into();
        match c {
            Character::Char(U_CHARACTER_TABULATION)
            | Character::Char(U_LINE_FEED)
            | Character::Char(U_FORM_FEED)
            | Character::Char(U_SPACE)
                if self.is_appropriate_end_tag_token(&last_start_tag_emitted) =>
            {
                States::before_attribute_name(self.token).into_transition_result()
            }
            Character::Char(U_SOLIDUS)
                if self.is_appropriate_end_tag_token(&last_start_tag_emitted) =>
            {
                States::self_closing_start_tag(self.token).into_transition_result()
            }
            Character::Char(U_GREATER_THAN_SIGN)
                if self.is_appropriate_end_tag_token(&last_start_tag_emitted) =>
            {
                let mut ret = States::data().into_transition_result();
                ret.push_emit(self.token);
                ret
            }
            Character::Char(c) if c.is_ascii_uppercase() => {
                self.token.push(c.to_lowercase().next().unwrap());
                self.tmp.push(c);

                States::from(self).into_transition_result()
            }
            Character::Char(c) if c.is_ascii_lowercase() => {
                self.token.push(c);
                self.tmp.push(c);

                States::from(self).into_transition_result()
            }
            _ => {
                // TODO ownership
                let tmp = self.tmp.clone();

                let mut ret = States::rc_data(self.tmp).into_transition_result();
                ret.push_emit(U_LESS_THAN_SIGN);
                ret.push_emit(U_SOLIDUS);
                for c in tmp.chars() {
                    ret.push_emit(c);
                }
                ret.set_reconsume();

                ret
            }
        }
    }

    // An appropriate end tag token is an end tag token whose tag name matches
    // the tag name of the last start tag to have been emitted from this
    // tokenizer, if any.
    // If no start tag has been emitted from this tokenizer, then no end tag
    // token is appropriate.
    fn is_appropriate_end_tag_token(&self, last_start_tag_emitted: &Option<StartTag>) -> bool {
        if let Token::EndTag(ref token) = self.token {
            if let Some(tag) = last_start_tag_emitted {
                if tag.name == token.name {
                    return true;
                }
            }
            false
        } else {
            panic!(
                "Unexpected token in RcDataEndTagName::is_appropriate_end_tag_token: {:?}",
                self.token
            );
        }
    }
}

impl RawTextLessThanSign {
    pub(super) fn on_character(mut self, c: Character) -> TransitionResult {
        #[allow(clippy::single_match_else)]
        match c {
            Character::Char(U_SOLIDUS) => {
                self.tmp = String::new();
                States::raw_text_end_tag_open(self.tmp).into_transition_result()
            }
            _ => {
                let mut ret = States::raw_text(self.tmp).into_transition_result();
                ret.push_emit(U_LESS_THAN_SIGN);
                ret.set_reconsume();
                ret
            }
        }
    }
}

impl RawTextEndTagOpen {
    pub(super) fn on_character(self, c: Character) -> TransitionResult {
        match c {
            Character::Char(a) if a.is_alphabetic() => {
                let token = EndTag::default();

                let mut ret =
                    States::raw_text_end_tag_name(token, self.tmp).into_transition_result();
                ret.set_reconsume();
                ret
            }
            _ => {
                let mut ret = States::raw_text(self.tmp).into_transition_result();
                ret.push_emit(U_LESS_THAN_SIGN);
                ret.push_emit(U_SOLIDUS);
                ret.set_reconsume();
                ret
            }
        }
    }
}

impl RawTextEndTagName {
    pub(super) fn on_character_and_last_start_tag(
        mut self,
        c: CharacterAndLastStartTag,
    ) -> TransitionResult {
        let (c, last_start_tag_emitted) = c.into();
        match c {
            Character::Char(U_CHARACTER_TABULATION)
            | Character::Char(U_LINE_FEED)
            | Character::Char(U_FORM_FEED)
            | Character::Char(U_SPACE)
                if self.is_appropriate_end_tag_token(&last_start_tag_emitted) =>
            {
                States::before_attribute_name(self.token).into_transition_result()
            }
            Character::Char(U_SOLIDUS)
                if self.is_appropriate_end_tag_token(&last_start_tag_emitted) =>
            {
                States::self_closing_start_tag(self.token).into_transition_result()
            }
            Character::Char(U_GREATER_THAN_SIGN)
                if self.is_appropriate_end_tag_token(&last_start_tag_emitted) =>
            {
                let mut ret = States::data().into_transition_result();
                ret.push_emit(self.token);
                ret
            }
            Character::Char(c) if c.is_ascii_uppercase() => {
                self.token.push(c.to_lowercase().next().unwrap());
                self.tmp.push(c);

                States::from(self).into_transition_result()
            }
            Character::Char(c) if c.is_ascii_lowercase() => {
                self.token.push(c);
                self.tmp.push(c);

                States::from(self).into_transition_result()
            }
            _ => {
                // TODO ownership
                let tmp = self.tmp.clone();

                let mut ret = States::raw_text(self.tmp).into_transition_result();
                ret.push_emit(U_LESS_THAN_SIGN);
                ret.push_emit(U_SOLIDUS);
                for c in tmp.chars() {
                    ret.push_emit(c);
                }
                ret.set_reconsume();

                ret
            }
        }
    }

    // An appropriate end tag token is an end tag token whose tag name matches
    // the tag name of the last start tag to have been emitted from this
    // tokenizer, if any.
    // If no start tag has been emitted from this tokenizer, then no end tag
    // token is appropriate.
    fn is_appropriate_end_tag_token(&self, last_start_tag_emitted: &Option<StartTag>) -> bool {
        if let Token::EndTag(ref token) = self.token {
            if let Some(tag) = last_start_tag_emitted {
                if tag.name == token.name {
                    return true;
                }
            }
            false
        } else {
            panic!(
                "Unexpected token in RawTextEndTagName::is_appropriate_end_tag_token: {:?}",
                self.token
            );
        }
    }
}

impl BeforeAttributeName {
    pub(super) fn on_character(mut self, c: Character) -> TransitionResult {
        match c {
            Character::Char(U_CHARACTER_TABULATION)
            | Character::Char(U_LINE_FEED)
            | Character::Char(U_FORM_FEED)
            | Character::Char(U_SPACE) => States::from(self).into_transition_result(),
            Character::Char(U_SOLIDUS) | Character::Char(U_GREATER_THAN_SIGN) | Character::Eof => {
                let mut ret = States::after_attribute_name(self.token).into_transition_result();
                ret.set_reconsume();
                ret
            }
            Character::Char(U_EQUALS_SIGN) => {
                self.token.add_attribute("=".to_string(), String::new());
                let mut ret = States::attribute_name(self.token).into_transition_result();
                ret.push_parse_error(ParseError::UnexpectedEqualsSignBeforeAttributeName);
                ret
            }
            _ => {
                self.token.add_attribute(String::new(), String::new());
                let mut ret = States::attribute_name(self.token).into_transition_result();
                ret.set_reconsume();
                ret
            }
        }
    }
}

impl AttributeName {
    pub(super) fn on_character(mut self, c: Character) -> TransitionResult {
        match c {
            Character::Char(U_CHARACTER_TABULATION)
            | Character::Char(U_LINE_FEED)
            | Character::Char(U_FORM_FEED)
            | Character::Char(U_SPACE)
            | Character::Char(U_SOLIDUS)
            | Character::Char(U_GREATER_THAN_SIGN)
            | Character::Eof => {
                self.check_duplicate_attribuite();

                let mut ret = States::after_attribute_name(self.token).into_transition_result();
                ret.set_reconsume();
                ret
            }
            Character::Char(U_EQUALS_SIGN) => {
                self.check_duplicate_attribuite();

                States::before_attribute_value(self.token).into_transition_result()
            }
            Character::Char(c) if c.is_ascii_uppercase() => {
                let attribute = self
                    .token
                    .current_attribute_mut()
                    .expect("No attribute found when handling AttributeName");
                attribute.push_name(c.to_lowercase().next().unwrap());

                States::from(self).into_transition_result()
            }
            Character::Char(U_NULL) => {
                let attribute = self
                    .token
                    .current_attribute_mut()
                    .expect("No attribute found when handling AttributeName");
                attribute.push_name(U_REPLACEMENT_CHARACTER);

                let mut ret = States::from(self).into_transition_result();
                ret.push_parse_error(ParseError::UnexpectedNullCharacter);
                ret
            }
            Character::Char(c) => {
                let attribute = self
                    .token
                    .current_attribute_mut()
                    .expect("No attribute found when handling AttributeName");
                attribute.push_name(c);

                let mut ret = States::from(self).into_transition_result();

                if c == U_QUOTATION_MARK || c == U_APOSTROPHE || c == U_LESS_THAN_SIGN {
                    ret.push_parse_error(ParseError::UnexpectedCharacterInAttributeName);
                }
                ret
            }
        }
    }

    fn check_duplicate_attribuite(&mut self) {
        if let Token::StartTag(ref mut tag) = self.token {
            let num_attributes = tag.attributes_iter().count();
            let current_attribute_name = if let Some(current_attribute) = tag.current_attribute() {
                current_attribute.name.to_owned()
            } else {
                return;
            };

            let duplicate = tag.attributes_iter().enumerate().any(|(n, attribute)| {
                // The last (current) attribute is never a duplicate
                if n == num_attributes - 1 {
                    return false;
                }

                if attribute.name == current_attribute_name {
                    return true;
                }
                false
            });
            if duplicate {
                if let Some(ref mut current_attribute) = tag.current_attribute_mut() {
                    trace!("Found duplicate attribute: {:?}", current_attribute);
                    current_attribute.set_duplicate()
                }
            }
        } else {
            panic!(
                "Unexpected token in AttributeName::check_duplicate_attribuite: {:?}",
                self.token
            );
        }
    }
}

impl AfterAttributeName {
    pub(super) fn on_character(mut self, c: Character) -> TransitionResult {
        match c {
            Character::Char(U_CHARACTER_TABULATION)
            | Character::Char(U_LINE_FEED)
            | Character::Char(U_FORM_FEED)
            | Character::Char(U_SPACE) => States::from(self).into_transition_result(),

            Character::Char(U_SOLIDUS) => {
                States::self_closing_start_tag(self.token).into_transition_result()
            }
            Character::Char(U_EQUALS_SIGN) => {
                States::attribute_value_single_quoted(self.token).into_transition_result()
            }
            Character::Char(U_GREATER_THAN_SIGN) => {
                let mut ret = States::data().into_transition_result();
                ret.push_emit(self.token);
                ret
            }
            Character::Eof => {
                let mut ret = States::term().into_transition_result();
                ret.push_parse_error(ParseError::EofInTag);
                ret.push_emit(Token::Eof);
                ret
            }
            _ => {
                self.token.add_attribute(String::new(), String::new());

                let mut ret = States::attribute_name(self.token).into_transition_result();
                ret.set_reconsume();
                ret
            }
        }
    }
}

impl BeforeAttributeValue {
    pub(super) fn on_character(self, c: Character) -> TransitionResult {
        match c {
            Character::Char(U_CHARACTER_TABULATION)
            | Character::Char(U_LINE_FEED)
            | Character::Char(U_FORM_FEED)
            | Character::Char(U_SPACE) => States::from(self).into_transition_result(),
            Character::Char(U_QUOTATION_MARK) => {
                States::attribute_value_double_quoted(self.token).into_transition_result()
            }
            Character::Char(U_APOSTROPHE) => {
                States::attribute_value_single_quoted(self.token).into_transition_result()
            }
            Character::Char(U_GREATER_THAN_SIGN) => {
                let mut ret = States::data().into_transition_result();
                ret.push_parse_error(ParseError::MissingAttributeValue);
                ret.push_emit(self.token);
                ret
            }
            _ => {
                let mut ret = States::attribute_value_unquoted(self.token).into_transition_result();
                ret.set_reconsume();
                ret
            }
        }
    }
}

impl AttributeValueDoubleQuoted {
    pub(super) fn on_character(mut self, c: Character) -> TransitionResult {
        match c {
            Character::Char(U_QUOTATION_MARK) => {
                States::after_attribute_value_quoted(self.token).into_transition_result()
            }
            Character::Char(U_AMPERSAND) => {
                States::character_reference(self, String::new()).into_transition_result()
            }
            Character::Char(U_NULL) => {
                let attribute = self
                    .token
                    .current_attribute_mut()
                    .expect("No attribute found when handling AttributeValueDoubleQuoted");
                attribute.push_value(U_REPLACEMENT_CHARACTER);

                let mut ret = States::from(self).into_transition_result();
                ret.push_parse_error(ParseError::UnexpectedNullCharacter);
                ret
            }
            Character::Eof => {
                let mut ret = States::term().into_transition_result();
                ret.push_parse_error(ParseError::EofInTag);
                ret.push_emit(self.token);
                ret.push_emit(Token::Eof);
                ret
            }
            Character::Char(c) => {
                let attribute = self
                    .token
                    .current_attribute_mut()
                    .expect("No attribute found when handling AttributeValueDoubleQuoted");
                attribute.push_value(c);

                States::from(self).into_transition_result()
            }
        }
    }
}

impl AttributeValueSingleQuoted {
    pub(super) fn on_character(mut self, c: Character) -> TransitionResult {
        match c {
            Character::Char(U_APOSTROPHE) => {
                States::after_attribute_value_quoted(self.token).into_transition_result()
            }
            Character::Char(U_AMPERSAND) => {
                States::character_reference(self, String::new()).into_transition_result()
            }
            Character::Char(U_NULL) => {
                let attribute = self
                    .token
                    .current_attribute_mut()
                    .expect("No attribute found when handling AttributeValueSingleQuoted");
                attribute.push_value(U_REPLACEMENT_CHARACTER);

                let mut ret = States::from(self).into_transition_result();
                ret.push_parse_error(ParseError::UnexpectedNullCharacter);
                ret
            }
            Character::Eof => {
                let mut ret = States::term().into_transition_result();
                ret.push_parse_error(ParseError::EofInTag);
                ret.push_emit(self.token);
                ret.push_emit(Token::Eof);
                ret
            }
            Character::Char(c) => {
                let attribute = self
                    .token
                    .current_attribute_mut()
                    .expect("No attribute found when handling AttributeValueSingleQuoted");
                attribute.push_value(c);

                States::from(self).into_transition_result()
            }
        }
    }
}

impl AttributeValueUnquoted {
    pub(super) fn on_character(mut self, c: Character) -> TransitionResult {
        match c {
            Character::Char(U_CHARACTER_TABULATION)
            | Character::Char(U_LINE_FEED)
            | Character::Char(U_FORM_FEED)
            | Character::Char(U_SPACE) => {
                States::before_attribute_name(self.token).into_transition_result()
            }
            Character::Char(U_AMPERSAND) => {
                States::character_reference(self, String::new()).into_transition_result()
            }
            Character::Char(U_GREATER_THAN_SIGN) => {
                let mut ret = States::data().into_transition_result();
                ret.push_emit(self.token);
                ret
            }
            Character::Char(U_NULL) => {
                let attribute = self
                    .token
                    .current_attribute_mut()
                    .expect("No attribute found when handling AttributeValueUnquoted");
                attribute.push_value(U_REPLACEMENT_CHARACTER);

                let mut ret = States::from(self).into_transition_result();
                ret.push_parse_error(ParseError::UnexpectedNullCharacter);
                ret
            }

            Character::Eof => {
                let mut ret = States::term().into_transition_result();
                ret.push_parse_error(ParseError::EofInTag);
                ret.push_emit(self.token);
                ret.push_emit(Token::Eof);
                ret
            }
            Character::Char(c) => {
                let attribute = self
                    .token
                    .current_attribute_mut()
                    .expect("No attribute found when handling AttributeValueUnquoted");
                attribute.push_value(c);

                let mut ret = States::from(self).into_transition_result();
                if c == U_QUOTATION_MARK
                    || c == U_APOSTROPHE
                    || c == U_LESS_THAN_SIGN
                    || c == U_EQUALS_SIGN
                    || c == U_GRAVE_ACCENT
                {
                    ret.push_parse_error(ParseError::UnexpectedCharacterInUnquotedAttributeValue);
                }
                ret
            }
        }
    }
}

impl AfterAttributeValueQuoted {
    pub(super) fn on_character(self, c: Character) -> TransitionResult {
        match c {
            Character::Char(U_CHARACTER_TABULATION)
            | Character::Char(U_LINE_FEED)
            | Character::Char(U_FORM_FEED)
            | Character::Char(U_SPACE) => {
                States::before_attribute_name(self.token).into_transition_result()
            }
            Character::Char(U_SOLIDUS) => {
                States::self_closing_start_tag(self.token).into_transition_result()
            }
            Character::Char(U_GREATER_THAN_SIGN) => {
                let mut ret = States::data().into_transition_result();
                ret.push_emit(self.token);
                ret
            }
            Character::Eof => {
                let mut ret = States::term().into_transition_result();
                ret.push_parse_error(ParseError::EofInTag);
                ret.push_emit(self.token);
                ret.push_emit(Token::Eof);
                ret
            }
            _ => {
                let mut ret = States::before_attribute_name(self.token).into_transition_result();
                ret.push_parse_error(ParseError::MissingWhitespaceBetweenAttributes);
                ret.set_reconsume();
                ret
            }
        }
    }
}

impl SelfClosingStartTag {
    pub(super) fn on_character(mut self, c: Character) -> TransitionResult {
        match c {
            Character::Char(U_GREATER_THAN_SIGN) => {
                self.token.set_self_closing(SelfClosingFlag::Set);

                let mut ret = States::data().into_transition_result();
                ret.push_emit(self.token);
                ret
            }
            Character::Eof => {
                let mut ret = States::term().into_transition_result();
                ret.push_parse_error(ParseError::EofInTag);
                ret.push_emit(Token::Eof);
                ret
            }
            _ => {
                let mut ret = States::before_attribute_name(self.token).into_transition_result();
                ret.push_parse_error(ParseError::UnexpectedSolidusInTag);
                ret.set_reconsume();
                ret
            }
        }
    }
}

impl BogusComment {
    pub(super) fn on_character(mut self, c: Character) -> TransitionResult {
        match c {
            Character::Char(U_GREATER_THAN_SIGN) => {
                let mut ret = States::data().into_transition_result();
                ret.push_emit(self.token);
                ret
            }
            Character::Eof => {
                let mut ret = States::term().into_transition_result();
                ret.push_emit(self.token);
                ret.push_emit(Token::Eof);
                ret
            }
            Character::Char(U_NULL) => {
                self.token.push(U_REPLACEMENT_CHARACTER);

                let mut ret = States::from(self).into_transition_result();
                ret.push_parse_error(ParseError::UnexpectedNullCharacter);
                ret
            }
            Character::Char(c) => {
                self.token.push(c);

                States::from(self).into_transition_result()
            }
        }
    }
}

impl MarkupDeclarationOpen {
    pub(super) fn on_next_few_characters(self, next: &NextFewCharacters) -> TransitionResult {
        if next.as_ref().is_none() {
            let mut ret = States::bogus_comment(String::new()).into_transition_result();
            ret.push_parse_error(ParseError::IncorrectlyOpenedComment);
            ret
        } else {
            match next.as_ref().as_ref().unwrap().as_str() {
                "DOCTYPE" => States::doctype().into_transition_result(),
                "--" => States::comment_start(String::new()).into_transition_result(),
                "[CDATA[" => {
                    // If there is an adjusted current node and it is not an element in the HTML namespace, then switch to the CDATA section state.
                    if let Some(_node) = self.get_adjusted_current_node() {
                        // if !node.is_element_in_html_namespace() {
                        return States::cdata_section().into_transition_result();
                        // }
                    }
                    //     Otherwise, this is a cdata-in-html-content parse error.
                    //     Create a comment token whose data is the "[CDATA[" string. Switch to the bogus comment state.
                    let mut ret = States::bogus_comment("[CDATA[").into_transition_result();
                    ret.push_parse_error(ParseError::CdataInHtmlContent);
                    ret
                }
                _ => unreachable!(),
            }
        }
    }

    fn get_adjusted_current_node(&self) -> Option<()> {
        todo!("MarkupDeclarationOpen::get_adjusted_current_node")
    }
}

impl CommentStart {
    pub(super) fn on_character(self, c: Character) -> TransitionResult {
        match c {
            Character::Char(U_HYPHEN_MINUS) => {
                States::comment_start_dash(self.token).into_transition_result()
            }
            Character::Char(U_GREATER_THAN_SIGN) => {
                let mut ret = States::data().into_transition_result();
                ret.push_parse_error(ParseError::AbruptClosingOfEmptyComment);
                ret.push_emit(self.token);
                ret
            }
            _ => {
                let mut ret = States::comment(self.token).into_transition_result();
                ret.set_reconsume();
                ret
            }
        }
    }
}

impl CommentStartDash {
    pub(super) fn on_character(mut self, c: Character) -> TransitionResult {
        match c {
            Character::Char(U_HYPHEN_MINUS) => {
                States::comment_end(self.token).into_transition_result()
            }
            Character::Char(U_GREATER_THAN_SIGN) => {
                let mut ret = States::data().into_transition_result();
                ret.push_parse_error(ParseError::AbruptClosingOfEmptyComment);
                ret.push_emit(self.token);
                ret
            }
            Character::Eof => {
                let mut ret = States::term().into_transition_result();
                ret.push_parse_error(ParseError::EofInComment);
                ret.push_emit(self.token);
                ret.push_emit(Token::Eof);
                ret
            }
            _ => {
                self.token.push(U_HYPHEN_MINUS);

                let mut ret = States::comment(self.token).into_transition_result();
                ret.set_reconsume();
                ret
            }
        }
    }
}

impl Comment {
    pub(super) fn on_character(mut self, c: Character) -> TransitionResult {
        match c {
            Character::Char(U_LESS_THAN_SIGN) => {
                self.token.push(U_LESS_THAN_SIGN);
                States::comment_less_than_sign(self.token).into_transition_result()
            }
            Character::Char(U_HYPHEN_MINUS) => {
                States::comment_end_dash(self.token).into_transition_result()
            }
            Character::Char(U_NULL) => {
                self.token.push(U_REPLACEMENT_CHARACTER);

                let mut ret = States::from(self).into_transition_result();
                ret.push_parse_error(ParseError::UnexpectedNullCharacter);
                ret
            }
            Character::Eof => {
                let mut ret = States::term().into_transition_result();
                ret.push_parse_error(ParseError::EofInComment);
                ret.push_emit(self.token);
                ret.push_emit(Token::Eof);
                ret
            }
            Character::Char(c) => {
                self.token.push(c);

                States::from(self).into_transition_result()
            }
        }
    }
}

impl CommentEndDash {
    pub(super) fn on_character(mut self, c: Character) -> TransitionResult {
        match c {
            Character::Char(U_HYPHEN_MINUS) => {
                States::comment_end(self.token).into_transition_result()
            }
            Character::Eof => {
                let mut ret = States::term().into_transition_result();
                ret.push_parse_error(ParseError::EofInTag);
                ret.push_emit(self.token);
                ret.push_emit(Token::Eof);
                ret
            }
            _ => {
                self.token.push(U_HYPHEN_MINUS);

                let mut ret = States::comment(self.token).into_transition_result();
                ret.set_reconsume();
                ret
            }
        }
    }
}

impl CommentEnd {
    pub(super) fn on_character(mut self, c: Character) -> TransitionResult {
        match c {
            Character::Char(U_GREATER_THAN_SIGN) => {
                let mut ret = States::data().into_transition_result();
                ret.push_emit(self.token);
                ret
            }
            Character::Char(U_EXCLAMATION_MARK) => {
                States::comment_end_bang(self.token).into_transition_result()
            }
            Character::Char(U_HYPHEN_MINUS) => {
                self.token.push(U_HYPHEN_MINUS);

                States::from(self).into_transition_result()
            }
            Character::Eof => {
                let mut ret = States::term().into_transition_result();
                ret.push_parse_error(ParseError::EofInTag);
                ret.push_emit(self.token);
                ret.push_emit(Token::Eof);
                ret
            }
            _ => {
                self.token.push(U_HYPHEN_MINUS);
                self.token.push(U_HYPHEN_MINUS);

                let mut ret = States::comment(self.token).into_transition_result();
                ret.set_reconsume();
                ret
            }
        }
    }
}

impl CommentEndBang {
    pub(super) fn on_character(mut self, c: Character) -> TransitionResult {
        match c {
            Character::Char(U_HYPHEN_MINUS) => {
                self.token.push(U_HYPHEN_MINUS);
                self.token.push(U_HYPHEN_MINUS);
                self.token.push(U_EXCLAMATION_MARK);
                States::comment_end_dash(self.token).into_transition_result()
            }
            Character::Char(U_GREATER_THAN_SIGN) => {
                let mut ret = States::data().into_transition_result();
                ret.push_parse_error(ParseError::IncorrectlyOpenedComment);
                ret.push_emit(self.token);
                ret
            }
            Character::Eof => {
                let mut ret = States::term().into_transition_result();
                ret.push_parse_error(ParseError::EofInComment);
                ret.push_emit(self.token);
                ret.push_emit(Token::Eof);
                ret
            }
            _ => {
                self.token.push(U_HYPHEN_MINUS);
                self.token.push(U_HYPHEN_MINUS);
                self.token.push(U_EXCLAMATION_MARK);

                let mut ret = States::comment(self.token).into_transition_result();
                ret.set_reconsume();
                ret
            }
        }
    }
}

impl Doctype {
    pub(super) fn on_character(self, c: Character) -> TransitionResult {
        match c {
            Character::Char(U_CHARACTER_TABULATION)
            | Character::Char(U_LINE_FEED)
            | Character::Char(U_FORM_FEED)
            | Character::Char(U_SPACE) => States::before_doctype_name().into_transition_result(),
            Character::Char(U_GREATER_THAN_SIGN) => {
                let mut ret = States::before_doctype_name().into_transition_result();
                ret.set_reconsume();
                ret
            }
            Character::Eof => {
                let token = DoctypeToken {
                    name: None,
                    force_quirks: ForceQuirksFlag::On,
                    ..DoctypeToken::default()
                };

                let mut ret = States::term().into_transition_result();
                ret.push_parse_error(ParseError::EofInDoctype);
                ret.push_emit(token);
                ret.push_emit(Token::Eof);
                ret
            }
            _ => {
                let mut ret = States::before_doctype_name().into_transition_result();
                ret.push_parse_error(ParseError::MissingWhitespaceBeforeDoctypeName);
                ret.set_reconsume();
                ret
            }
        }
    }
}

impl BeforeDoctypeName {
    pub(super) fn on_character(self, c: Character) -> TransitionResult {
        match c {
            Character::Char(U_CHARACTER_TABULATION)
            | Character::Char(U_LINE_FEED)
            | Character::Char(U_FORM_FEED)
            | Character::Char(U_SPACE) => States::from(self).into_transition_result(),
            Character::Char(c) if c.is_ascii_uppercase() => {
                let token = DoctypeToken::from_char(c.to_lowercase().next().unwrap());
                States::doctype_name(token).into_transition_result()
            }
            Character::Char(U_NULL) => {
                let token = DoctypeToken::from_char(U_REPLACEMENT_CHARACTER);

                let mut ret = States::doctype_name(token).into_transition_result();
                ret.push_parse_error(ParseError::UnexpectedNullCharacter);
                ret
            }
            Character::Char(U_GREATER_THAN_SIGN) => {
                let token = DoctypeToken {
                    name: None,
                    force_quirks: ForceQuirksFlag::On,
                    ..DoctypeToken::default()
                };

                let mut ret = States::data().into_transition_result();
                ret.push_parse_error(ParseError::MissingDoctypeName);
                ret.push_emit(token);
                ret
            }
            Character::Eof => {
                let token = DoctypeToken {
                    name: None,
                    force_quirks: ForceQuirksFlag::On,
                    ..DoctypeToken::default()
                };

                let mut ret = States::term().into_transition_result();
                ret.push_parse_error(ParseError::EofInDoctype);
                ret.push_emit(token);
                ret.push_emit(Token::Eof);
                ret
            }
            Character::Char(c) => {
                let token = DoctypeToken::from_char(c);
                States::doctype_name(token).into_transition_result()
            }
        }
    }
}

impl DoctypeName {
    pub(super) fn on_character(mut self, c: Character) -> TransitionResult {
        match c {
            Character::Char(U_CHARACTER_TABULATION)
            | Character::Char(U_LINE_FEED)
            | Character::Char(U_FORM_FEED)
            | Character::Char(U_SPACE) => States::after_doctype_name().into_transition_result(),
            Character::Char(U_GREATER_THAN_SIGN) => {
                let mut ret = States::data().into_transition_result();
                ret.push_emit(self.token);
                ret
            }
            Character::Char(c) if c.is_ascii_uppercase() => {
                self.token.push(c.to_lowercase().next().unwrap());

                States::from(self).into_transition_result()
            }
            Character::Char(U_NULL) => {
                self.token.push(U_REPLACEMENT_CHARACTER);

                let mut ret = States::from(self).into_transition_result();
                ret.push_parse_error(ParseError::UnexpectedNullCharacter);
                ret
            }
            Character::Eof => {
                self.token.set_force_quirks(ForceQuirksFlag::On);

                let mut ret = States::term().into_transition_result();
                ret.push_parse_error(ParseError::EofInDoctype);
                ret.push_emit(self.token);
                ret.push_emit(Token::Eof);
                ret
            }
            Character::Char(c) => {
                self.token.push(c);

                States::from(self).into_transition_result()
            }
        }
    }
}

impl CharacterReference {
    pub(super) fn on_character(mut self, c: Character) -> TransitionResult {
        self.tmp = String::new();
        self.tmp.push(U_AMPERSAND);
        match c {
            Character::Char(a) if a.is_alphanumeric() => {
                // Reconsume in the named character reference state.
                let mut ret = States::named_character_reference(self.return_state, self.tmp)
                    .into_transition_result();
                ret.set_reconsume();
                ret
            }
            Character::Char(U_NUMBER_SIGN) => {
                self.tmp.push(U_NUMBER_SIGN);
                States::numeric_character_reference(self.return_state, self.tmp)
                    .into_transition_result()
            }
            _ => {
                // TODO: ownership...
                let tmp = self.tmp.clone();
                let to_emit = flush_codepoints_consumed_as_character_reference(
                    self.get_attribute_token(),
                    &tmp,
                );

                let mut ret = self.return_state.into_transition_result();
                for emit in to_emit {
                    ret.push_emit(emit);
                }
                ret.set_reconsume();
                ret
            }
        }
    }

    fn get_attribute_token(&mut self) -> Option<&mut Token> {
        match *self.return_state {
            States::AttributeValueDoubleQuoted(AttributeValueDoubleQuoted { ref mut token })
            | States::AttributeValueSingleQuoted(AttributeValueSingleQuoted { ref mut token })
            | States::AttributeValueUnquoted(AttributeValueUnquoted { ref mut token }) => {
                Some(token)
            }
            _ => None,
        }
    }
}

impl NamedCharacterReference {
    pub(super) fn on_possible_character_reference_with_next_char(
        mut self,
        input: PossibleCharacterReferenceWithNextChar,
    ) -> TransitionResult {
        let (possible_char_ref, next_c) = input.into();

        let next_char_equals_or_alpha = match next_c {
            Character::Char(U_EQUALS_SIGN) => true,
            Character::Char(ch) if ch.is_alphanumeric() => true,
            _ => false,
        };

        // There was a match
        if let Some(char_ref) = possible_char_ref {
            // If the character reference was consumed as part of an attribute,
            // and the last character matched is not a U+003B SEMICOLON character (;),
            // and the next input character is either a U+003D EQUALS SIGN character
            // (=) or an ASCII alphanumeric, then, for historical reasons,
            // flush code points consumed as a character reference and switch to the return state.
            let was_consumed_as_part_of_attribute = self.get_attribute_token().is_some();
            let last_char_is_semicolon = char_ref.ends_with(U_SEMICOLON);
            let historical = was_consumed_as_part_of_attribute
                && last_char_is_semicolon
                && next_char_equals_or_alpha;

            if historical {
                // flush code points consumed as a character reference and switch to the return state.
                trace!("Skipping char named character lookup for historical reasons");

                // TODO: ownership...
                let tmp = self.tmp.clone();
                let to_emit = flush_codepoints_consumed_as_character_reference(
                    self.get_attribute_token(),
                    &tmp,
                );

                let mut ret = self.return_state.into_transition_result();
                for emit in to_emit {
                    ret.push_emit(emit);
                }
                ret
            } else {
                // 1. If the last character matched is not a U+003B SEMICOLON character (;),
                //    then this is a missing-semicolon-after-character-reference parse error.
                // 2. Set the temporary buffer to the empty string.
                //    Append one or two characters corresponding to the character reference name
                //    (as given by the second column of the named character references table) to the temporary buffer.
                // 3. Flush code points consumed as a character reference. Switch to the return state.
                self.tmp = String::new();
                self.tmp.push_str(
                    get_entities()
                        .get(char_ref.as_str())
                        .unwrap()
                        .characters
                        .as_str(),
                );
                // TODO: ownership...
                let tmp = self.tmp.clone();
                let to_emit = flush_codepoints_consumed_as_character_reference(
                    self.get_attribute_token(),
                    &tmp,
                );

                let mut ret = self.return_state.into_transition_result();

                if !last_char_is_semicolon {
                    ret.push_parse_error(ParseError::MissingSemicolonAfterCharacterReference);
                }
                for emit in to_emit {
                    ret.push_emit(emit);
                }
                ret
            }
        } else {
            // Flush code points consumed as a character reference. Switch to the ambiguous ampersand state.
            // TODO: ownership...
            let tmp = self.tmp.clone();
            let to_emit =
                flush_codepoints_consumed_as_character_reference(self.get_attribute_token(), &tmp);

            let mut ret = States::ambiguous_ampersand(self.return_state).into_transition_result();
            for emit in to_emit {
                ret.push_emit(emit);
            }
            ret
        }
    }

    fn get_attribute_token(&mut self) -> Option<&mut Token> {
        match *self.return_state {
            States::AttributeValueDoubleQuoted(AttributeValueDoubleQuoted { ref mut token })
            | States::AttributeValueSingleQuoted(AttributeValueSingleQuoted { ref mut token })
            | States::AttributeValueUnquoted(AttributeValueUnquoted { ref mut token }) => {
                Some(token)
            }
            _ => None,
        }
    }
}

impl AmbiguousAmpersand {
    pub(super) fn on_character(mut self, c: Character) -> TransitionResult {
        match c {
            Character::Char(a) if a.is_alphanumeric() => {
                if let Some(token) = self.get_attribute_token() {
                    let attribute = token
                        .current_attribute_mut()
                        .expect("No attribute found when handling AmbiguousAmpersand");
                    attribute.push_value(a);

                    States::from(self).into_transition_result()
                } else {
                    let mut ret = States::from(self).into_transition_result();
                    ret.push_emit(a);
                    ret
                }
            }
            Character::Char(U_SEMICOLON) => {
                let mut ret = self.return_state.into_transition_result();
                ret.push_parse_error(ParseError::UnknownNamedCharacterReference);
                ret.set_reconsume();
                ret
            }
            _ => {
                let mut ret = self.return_state.into_transition_result();
                ret.set_reconsume();
                ret
            }
        }
    }

    fn get_attribute_token(&mut self) -> Option<&mut Token> {
        match *self.return_state {
            States::AttributeValueDoubleQuoted(AttributeValueDoubleQuoted { ref mut token })
            | States::AttributeValueSingleQuoted(AttributeValueSingleQuoted { ref mut token })
            | States::AttributeValueUnquoted(AttributeValueUnquoted { ref mut token }) => {
                Some(token)
            }
            _ => None,
        }
    }
}

impl NumericCharacterReference {
    pub(super) fn on_character(mut self, c: Character) -> TransitionResult {
        let character_reference_code = 0;
        match c {
            Character::Char(ch @ U_LATIN_SMALL_LETTER_X)
            | Character::Char(ch @ U_LATIN_CAPITAL_LETTER_X) => {
                self.tmp.push(ch);
                States::hexadecimal_character_reference_start(
                    self.return_state,
                    self.tmp,
                    character_reference_code,
                )
                .into_transition_result()
            }
            _ => {
                let mut ret = States::decimal_character_reference_start(
                    self.return_state,
                    self.tmp,
                    character_reference_code,
                )
                .into_transition_result();
                ret.set_reconsume();
                ret
            }
        }
    }
}

impl HexadecimalCharacterReferenceStart {
    pub(super) fn on_character(mut self, c: Character) -> TransitionResult {
        match c {
            Character::Char(ch) if ch.is_ascii_hexdigit() => {
                let mut ret = States::hexadecimal_character_reference(
                    self.return_state,
                    self.tmp,
                    self.character_reference_code,
                )
                .into_transition_result();
                ret.set_reconsume();
                ret
            }
            _ => {
                // TODO: ownership...
                let tmp = self.tmp.clone();
                let to_emit = flush_codepoints_consumed_as_character_reference(
                    self.get_attribute_token(),
                    &tmp,
                );

                let mut ret = self.return_state.into_transition_result();
                for emit in to_emit {
                    ret.push_emit(emit);
                }
                ret.push_parse_error(ParseError::AbsenceOfDigitsInNumericCharacterReference);
                ret.set_reconsume();
                ret
            }
        }
    }

    fn get_attribute_token(&mut self) -> Option<&mut Token> {
        match *self.return_state {
            States::AttributeValueDoubleQuoted(AttributeValueDoubleQuoted { ref mut token })
            | States::AttributeValueSingleQuoted(AttributeValueSingleQuoted { ref mut token })
            | States::AttributeValueUnquoted(AttributeValueUnquoted { ref mut token }) => {
                Some(token)
            }
            _ => None,
        }
    }
}

impl DecimalCharacterReferenceStart {
    pub(super) fn on_character(mut self, c: Character) -> TransitionResult {
        match c {
            Character::Char(ch) if ch.is_ascii_digit() => {
                let mut ret = States::decimal_character_reference(
                    self.return_state,
                    self.tmp,
                    self.character_reference_code,
                )
                .into_transition_result();
                ret.set_reconsume();
                ret
            }
            _ => {
                // TODO: ownership...
                let tmp = self.tmp.clone();
                let to_emit = flush_codepoints_consumed_as_character_reference(
                    self.get_attribute_token(),
                    &tmp,
                );

                let mut ret = self.return_state.into_transition_result();
                for emit in to_emit {
                    ret.push_emit(emit);
                }
                ret.push_parse_error(ParseError::AbsenceOfDigitsInNumericCharacterReference);
                ret.set_reconsume();
                ret
            }
        }
    }

    fn get_attribute_token(&mut self) -> Option<&mut Token> {
        match *self.return_state {
            States::AttributeValueDoubleQuoted(AttributeValueDoubleQuoted { ref mut token })
            | States::AttributeValueSingleQuoted(AttributeValueSingleQuoted { ref mut token })
            | States::AttributeValueUnquoted(AttributeValueUnquoted { ref mut token }) => {
                Some(token)
            }
            _ => None,
        }
    }
}

impl HexadecimalCharacterReference {
    pub(super) fn on_character(mut self, c: Character) -> TransitionResult {
        match c {
            Character::Char(ch) if ch.is_ascii_digit() => {
                self.character_reference_code *= 16;
                self.character_reference_code += ch.to_digit(10).unwrap();

                States::from(self).into_transition_result()
            }
            Character::Char(ch)
                if codepoint::is_ascii_upper_hex_digit(ch as codepoint::Codepoint) =>
            {
                self.character_reference_code *= 16;
                self.character_reference_code += ch.to_digit(16).unwrap();

                States::from(self).into_transition_result()
            }
            Character::Char(ch)
                if codepoint::is_ascii_lower_hex_digit(ch as codepoint::Codepoint) =>
            {
                self.character_reference_code *= 16;
                self.character_reference_code += ch.to_digit(16).unwrap();

                States::from(self).into_transition_result()
            }
            Character::Char(U_SEMICOLON) => States::numeric_character_reference_end(
                self.return_state,
                self.tmp,
                self.character_reference_code,
            )
            .into_transition_result(),
            _ => {
                let mut ret = States::numeric_character_reference_end(
                    self.return_state,
                    self.tmp,
                    self.character_reference_code,
                )
                .into_transition_result();

                ret.push_parse_error(ParseError::MissingSemicolonAfterCharacterReference);
                ret.set_reconsume();
                ret
            }
        }
    }
}

impl DecimalCharacterReference {
    pub(super) fn on_character(mut self, c: Character) -> TransitionResult {
        match c {
            Character::Char(ch) if ch.is_ascii_digit() => {
                self.character_reference_code *= 10;
                self.character_reference_code += ch.to_digit(10).unwrap();

                States::from(self).into_transition_result()
            }
            Character::Char(U_SEMICOLON) => States::numeric_character_reference_end(
                self.return_state,
                self.tmp,
                self.character_reference_code,
            )
            .into_transition_result(),
            _ => {
                let mut ret = States::numeric_character_reference_end(
                    self.return_state,
                    self.tmp,
                    self.character_reference_code,
                )
                .into_transition_result();

                ret.push_parse_error(ParseError::MissingSemicolonAfterCharacterReference);
                ret.set_reconsume();
                ret
            }
        }
    }
}

impl NumericCharacterReferenceEnd {
    pub(super) fn on_advance(mut self) -> TransitionResult {
        let (parse_err, character_reference_code) = match self.character_reference_code {
            0x00 => (Some(ParseError::NullCharacterReference), 0xFFFD),
            c if codepoint::is_surrogate(c) => (
                Some(ParseError::CharacterReferenceOutsideUnicodeRange),
                0xFFFD,
            ),
            c if codepoint::is_noncharacter(c) => {
                (Some(ParseError::NoncharacterCharacterReference), 0xFFFD)
            }
            c if (c == 0x0D
                || (codepoint::is_control(c) && !codepoint::is_ascii_whitespace(c))) =>
            {
                (
                    Some(ParseError::ControlCharacterReference),
                    NumericCharacterReferenceEnd::translate(c).unwrap_or(c),
                )
            }
            c => (None, c),
        };

        self.tmp = String::new();
        self.tmp
            .push(std::char::from_u32(character_reference_code).unwrap());

        // TODO: ownership...
        let tmp = self.tmp.clone();
        let to_emit =
            flush_codepoints_consumed_as_character_reference(self.get_attribute_token(), &tmp);

        let mut ret = self.return_state.into_transition_result();
        // TODO: ordering?
        for emit in to_emit {
            ret.push_emit(emit);
        }
        if let Some(parse_err) = parse_err {
            ret.push_parse_error(parse_err);
        }

        ret
    }

    fn get_attribute_token(&mut self) -> Option<&mut Token> {
        match *self.return_state {
            States::AttributeValueDoubleQuoted(AttributeValueDoubleQuoted { ref mut token })
            | States::AttributeValueSingleQuoted(AttributeValueSingleQuoted { ref mut token })
            | States::AttributeValueUnquoted(AttributeValueUnquoted { ref mut token }) => {
                Some(token)
            }
            _ => None,
        }
    }

    fn translate(c: CharacterReferenceCode) -> Option<CharacterReferenceCode> {
        match c {
            // EURO SIGN (€)
            0x80 => Some(0x20AC),
            // SINGLE LOW-9 QUOTATION MARK (‚)
            0x82 => Some(0x201A),
            // LATIN SMALL LETTER F WITH HOOK (ƒ)
            0x83 => Some(0x0192),
            // DOUBLE LOW-9 QUOTATION MARK („)
            0x84 => Some(0x201E),
            // HORIZONTAL ELLIPSIS (…)
            0x85 => Some(0x2026),
            // DAGGER (†)
            0x86 => Some(0x2020),
            // DOUBLE DAGGER (‡)
            0x87 => Some(0x2021),
            // MODIFIER LETTER CIRCUMFLEX ACCENT (ˆ)
            0x88 => Some(0x02C6),
            // PER MILLE SIGN (‰)
            0x89 => Some(0x2030),
            // LATIN CAPITAL LETTER S WITH CARON (Š)
            0x8A => Some(0x0160),
            // SINGLE LEFT-POINTING ANGLE QUOTATION MARK (‹)
            0x8B => Some(0x2039),
            // LATIN CAPITAL LIGATURE OE (Œ)
            0x8C => Some(0x0152),
            // LATIN CAPITAL LETTER Z WITH CARON (Ž)
            0x8E => Some(0x017D),
            // LEFT SINGLE QUOTATION MARK (‘)
            0x91 => Some(0x2018),
            // RIGHT SINGLE QUOTATION MARK (’)
            0x92 => Some(0x2019),
            // LEFT DOUBLE QUOTATION MARK (“)
            0x93 => Some(0x201C),
            // RIGHT DOUBLE QUOTATION MARK (”)
            0x94 => Some(0x201D),
            // BULLET (•)
            0x95 => Some(0x2022),
            // EN DASH (–)
            0x96 => Some(0x2013),
            // EM DASH (—)
            0x97 => Some(0x2014),
            // SMALL TILDE (˜)
            0x98 => Some(0x02DC),
            // TRADE MARK SIGN (™)
            0x99 => Some(0x2122),
            // LATIN SMALL LETTER S WITH CARON (š)
            0x9A => Some(0x0161),
            // SINGLE RIGHT-POINTING ANGLE QUOTATION MARK (›)
            0x9B => Some(0x203A),
            // LATIN SMALL LIGATURE OE (œ)
            0x9C => Some(0x0153),
            // LATIN SMALL LETTER Z WITH CARON (ž)
            0x9E => Some(0x017E),
            // LATIN CAPITAL LETTER Y WITH DIAERESIS (Ÿ)
            0x9F => Some(0x0178),

            _ => None,
        }
    }
}

/*
 * Transition Helpers
 */

fn flush_codepoints_consumed_as_character_reference(
    attribute_token: Option<&mut Token>,
    tmp: &str,
) -> Vec<Token> {
    trace!(
        "flush_codepoints_consumed_as_character_reference(tmp: {:?})",
        tmp
    );
    let mut to_emit = Vec::new();
    let chars = tmp.chars().collect::<Vec<_>>();

    if let Some(token) = attribute_token {
        let attribute = token.current_attribute_mut().unwrap();
        for c in chars {
            attribute.push_value(c);
        }
    } else {
        for c in chars {
            to_emit.push(c.into());
        }
    }
    to_emit
}
