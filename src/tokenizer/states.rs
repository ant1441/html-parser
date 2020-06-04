use derive_more::{AsRef, Deref, DerefMut, Display, From};

use super::{errors, token, Codepoint, TransitionResult};

macro_rules! create_states {
    ($($s:ident,)+) => {
        // #[derive(Copy, Clone, Debug, PartialEq, Eq, Display)]
         #[derive(Debug, PartialEq, Eq, Display)]
        pub enum States {
            $(
                $s($s),
            )*
        }

        $(
            // #[derive(Copy, Clone, Debug, PartialEq, Eq)]
            // pub struct $s {}

            impl ::std::fmt::Display for $s {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, stringify!($s))
                }
            }

            impl From<$s> for States {
                fn from(state: $s) -> Self {
                    States::$s(state)
                }
            }
        )*
    }
}

create_states! {
    Data,
    RcData,
    RawText,
    ScriptData,
    PlainText,
    TagOpen,
    EndTagOpen,
    TagName,
    RcDataLessThanSign,
    RcDataEndTagOpen,
    RcDataEndTagName,
    RawTextLessThanSign,
    RawTextEndTagOpen,
    RawTextEndTagName,
    ScriptDataLessThanSign,
    ScriptDataEndTagOpen,
    ScriptDataEndTagName,
    ScriptDataEscapeStart,
    ScriptDataEscapeStartDash,
    ScriptDataEscaped,
    ScriptDataEscapedDash,
    ScriptDataEscapedDashDash,
    ScriptDataEscapedLessThanSign,
    ScriptDataEscapedEndTagOpen,
    ScriptDataEscapedEndTagName,
    ScriptDataDoubleEscapeStart,
    ScriptDataDoubleEscaped,
    ScriptDataDoubleEscapedDash,
    ScriptDataDoubleEscapedDashDash,
    ScriptDataDoubleEscapedLessThanSign,
    ScriptDataDoubleEscapeEnd,
    BeforeAttributeName,
    AttributeName,
    AfterAttributeName,
    BeforeAttributeValue,
    AttributeValueDoubleQuoted,
    AttributeValueSingleQuoted,
    AttributeValueUnquoted,
    AfterAttributeValueQuoted,
    SelfClosingStartTag,
    BogusComment,
    MarkupDeclarationOpen,
    CommentStart,
    CommentStartDash,
    Comment,
    CommentLessThanSign,
    CommentLessThanSignBang,
    CommentLessThanSignBangDash,
    CommentLessThanSignBangDashDash,
    CommentEndDash,
    CommentEnd,
    CommentEndBang,
    Doctype,
    BeforeDoctypeName,
    DoctypeName,
    AfterDoctypeName,
    AfterDoctypePublicKeyword,
    BeforeDoctypePublicIdentifier,
    DoctypePublicIdentifierDoubleQuoted,
    DoctypePublicIdentifierSingleQuoted,
    AfterDoctypePublicIdentifier,
    BetweenDoctypePublicAndSystemIdentifiers,
    AfterDoctypeSystemKeyword,
    BeforeDoctypeSystemIdentifier,
    DoctypeSystemIdentifierDoubleQuoted,
    DoctypeSystemIdentifierSingleQuoted,
    AfterDoctypeSystemIdentifier,
    BogusDoctype,
    CdataSection,
    CdataSectionBracket,
    CdataSectionEnd,

    CharacterReference,

    NamedCharacterReference,
    AmbiguousAmpersand,
    NumericCharacterReference,
    HexadecimalCharacterReferenceStart,
    DecimalCharacterReferenceStart,
    HexadecimalCharacterReference,
    DecimalCharacterReference,
    NumericCharacterReferenceEnd,

    Term,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Data {}

#[derive(Debug, PartialEq, Eq)]
pub struct RcData {}

#[derive(Debug, PartialEq, Eq)]
pub struct RawText {}

#[derive(Debug, PartialEq, Eq)]
pub struct ScriptData {}

#[derive(Debug, PartialEq, Eq)]
pub struct PlainText {}

#[derive(Debug, PartialEq, Eq)]
pub struct TagOpen {}

#[derive(Debug, PartialEq, Eq)]
pub struct EndTagOpen {}

#[derive(Debug, PartialEq, Eq)]
pub struct TagName {
    pub(crate) token: token::Token,
}

#[derive(Debug, PartialEq, Eq)]
pub struct RcDataLessThanSign {}

#[derive(Debug, PartialEq, Eq)]
pub struct RcDataEndTagOpen {}

#[derive(Debug, PartialEq, Eq)]
pub struct RcDataEndTagName {
    pub(crate) token: token::Token,
    pub(crate) tmp: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct RawTextLessThanSign {}

#[derive(Debug, PartialEq, Eq)]
pub struct RawTextEndTagOpen {}

#[derive(Debug, PartialEq, Eq)]
pub struct RawTextEndTagName {}

#[derive(Debug, PartialEq, Eq)]
pub struct ScriptDataLessThanSign {}

#[derive(Debug, PartialEq, Eq)]
pub struct ScriptDataEndTagOpen {}

#[derive(Debug, PartialEq, Eq)]
pub struct ScriptDataEndTagName {}

#[derive(Debug, PartialEq, Eq)]
pub struct ScriptDataEscapeStart {}

#[derive(Debug, PartialEq, Eq)]
pub struct ScriptDataEscapeStartDash {}

#[derive(Debug, PartialEq, Eq)]
pub struct ScriptDataEscaped {}

#[derive(Debug, PartialEq, Eq)]
pub struct ScriptDataEscapedDash {}

#[derive(Debug, PartialEq, Eq)]
pub struct ScriptDataEscapedDashDash {}

#[derive(Debug, PartialEq, Eq)]
pub struct ScriptDataEscapedLessThanSign {}

#[derive(Debug, PartialEq, Eq)]
pub struct ScriptDataEscapedEndTagOpen {}

#[derive(Debug, PartialEq, Eq)]
pub struct ScriptDataEscapedEndTagName {}

#[derive(Debug, PartialEq, Eq)]
pub struct ScriptDataDoubleEscapeStart {}

#[derive(Debug, PartialEq, Eq)]
pub struct ScriptDataDoubleEscaped {}

#[derive(Debug, PartialEq, Eq)]
pub struct ScriptDataDoubleEscapedDash {}

#[derive(Debug, PartialEq, Eq)]
pub struct ScriptDataDoubleEscapedDashDash {}

#[derive(Debug, PartialEq, Eq)]
pub struct ScriptDataDoubleEscapedLessThanSign {}

#[derive(Debug, PartialEq, Eq)]
pub struct ScriptDataDoubleEscapeEnd {}

#[derive(Debug, PartialEq, Eq)]
pub struct BeforeAttributeName {
    pub(crate) token: token::Token,
}

#[derive(Debug, PartialEq, Eq)]
pub struct AttributeName {
    pub(crate) token: token::Token,
}

#[derive(Debug, PartialEq, Eq)]
pub struct AfterAttributeName {
    pub(crate) token: token::Token,
}

#[derive(Debug, PartialEq, Eq)]
pub struct BeforeAttributeValue {
    pub(crate) token: token::Token,
}

#[derive(Debug, PartialEq, Eq)]
pub struct AttributeValueDoubleQuoted {
    pub(crate) token: token::Token,
}

#[derive(Debug, PartialEq, Eq)]
pub struct AttributeValueSingleQuoted {
    pub(crate) token: token::Token,
}

#[derive(Debug, PartialEq, Eq)]
pub struct AttributeValueUnquoted {
    pub(crate) token: token::Token,
}

#[derive(Debug, PartialEq, Eq)]
pub struct AfterAttributeValueQuoted {
    pub(crate) token: token::Token,
}

#[derive(Debug, PartialEq, Eq)]
pub struct SelfClosingStartTag {
    pub(crate) token: token::Token,
}

#[derive(Debug, PartialEq, Eq)]
pub struct BogusComment {
    pub(crate) token: token::Token,
}

#[derive(Debug, PartialEq, Eq)]
pub struct MarkupDeclarationOpen {}

#[derive(Debug, PartialEq, Eq)]
pub struct CommentStart {
    pub(crate) token: token::Token,
}

#[derive(Debug, PartialEq, Eq)]
pub struct CommentStartDash {
    pub(crate) token: token::Token,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Comment {
    pub(crate) token: token::Token,
}

#[derive(Debug, PartialEq, Eq)]
pub struct CommentLessThanSign {
    pub(crate) token: token::Token,
}

#[derive(Debug, PartialEq, Eq)]
pub struct CommentLessThanSignBang {}

#[derive(Debug, PartialEq, Eq)]
pub struct CommentLessThanSignBangDash {}

#[derive(Debug, PartialEq, Eq)]
pub struct CommentLessThanSignBangDashDash {}

#[derive(Debug, PartialEq, Eq)]
pub struct CommentEndDash {
    pub(crate) token: token::Token,
}

#[derive(Debug, PartialEq, Eq)]
pub struct CommentEnd {
    pub(crate) token: token::Token,
}

#[derive(Debug, PartialEq, Eq)]
pub struct CommentEndBang {
    pub(crate) token: token::Token,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Doctype {}

#[derive(Debug, PartialEq, Eq)]
pub struct BeforeDoctypeName {}

#[derive(Debug, PartialEq, Eq)]
pub struct DoctypeName {
    pub(crate) token: token::Token,
}

#[derive(Debug, PartialEq, Eq)]
pub struct AfterDoctypeName {}

#[derive(Debug, PartialEq, Eq)]
pub struct AfterDoctypePublicKeyword {}

#[derive(Debug, PartialEq, Eq)]
pub struct BeforeDoctypePublicIdentifier {}

#[derive(Debug, PartialEq, Eq)]
pub struct DoctypePublicIdentifierDoubleQuoted {}

#[derive(Debug, PartialEq, Eq)]
pub struct DoctypePublicIdentifierSingleQuoted {}

#[derive(Debug, PartialEq, Eq)]
pub struct AfterDoctypePublicIdentifier {}

#[derive(Debug, PartialEq, Eq)]
pub struct BetweenDoctypePublicAndSystemIdentifiers {}

#[derive(Debug, PartialEq, Eq)]
pub struct AfterDoctypeSystemKeyword {}

#[derive(Debug, PartialEq, Eq)]
pub struct BeforeDoctypeSystemIdentifier {}

#[derive(Debug, PartialEq, Eq)]
pub struct DoctypeSystemIdentifierDoubleQuoted {}

#[derive(Debug, PartialEq, Eq)]
pub struct DoctypeSystemIdentifierSingleQuoted {}

#[derive(Debug, PartialEq, Eq)]
pub struct AfterDoctypeSystemIdentifier {}

#[derive(Debug, PartialEq, Eq)]
pub struct BogusDoctype {}

#[derive(Debug, PartialEq, Eq)]
pub struct CdataSection {}

#[derive(Debug, PartialEq, Eq)]
pub struct CdataSectionBracket {}

#[derive(Debug, PartialEq, Eq)]
pub struct CdataSectionEnd {}

#[derive(Debug, PartialEq, Eq)]
pub struct CharacterReference {
    pub(crate) return_state: Box<States>,
    pub(crate) tmp: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct NamedCharacterReference {
    pub(crate) return_state: Box<States>,
    pub(crate) tmp: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct AmbiguousAmpersand {
    pub(crate) return_state: Box<States>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct NumericCharacterReference {
    pub(crate) return_state: Box<States>,
    pub(crate) tmp: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct HexadecimalCharacterReferenceStart {
    pub(crate) tmp: String,
    pub(crate) return_state: Box<States>,
    pub(crate) character_reference_code: CharacterReferenceCode,
}

#[derive(Debug, PartialEq, Eq)]
pub struct DecimalCharacterReferenceStart {
    pub(crate) tmp: String,
    pub(crate) return_state: Box<States>,
    pub(crate) character_reference_code: CharacterReferenceCode,
}

#[derive(Debug, PartialEq, Eq)]
pub struct HexadecimalCharacterReference {
    pub(crate) tmp: String,
    pub(crate) return_state: Box<States>,
    pub(crate) character_reference_code: CharacterReferenceCode,
}

#[derive(Debug, PartialEq, Eq)]
pub struct DecimalCharacterReference {
    pub(crate) tmp: String,
    pub(crate) return_state: Box<States>,
    pub(crate) character_reference_code: CharacterReferenceCode,
}

#[derive(Debug, PartialEq, Eq)]
pub struct NumericCharacterReferenceEnd {
    pub(crate) tmp: String,
    pub(crate) return_state: Box<States>,
    pub(crate) character_reference_code: CharacterReferenceCode,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Term {}

pub(crate) type CharacterReferenceCode = Codepoint;

impl States {
    pub fn new() -> Self {
        States::data()
    }

    pub fn data() -> States {
        States::Data(Data {})
    }

    pub fn rc_data() -> States {
        States::RcData(RcData {})
    }

    pub fn raw_text() -> States {
        States::RawText(RawText {})
    }

    pub fn script_data() -> States {
        States::ScriptData(ScriptData {})
    }

    pub fn plain_text() -> States {
        States::PlainText(PlainText {})
    }

    pub fn tag_open() -> States {
        States::TagOpen(TagOpen {})
    }

    pub fn end_tag_open() -> States {
        States::EndTagOpen(EndTagOpen {})
    }

    pub fn tag_name(token: token::Token) -> States {
        States::TagName(TagName { token })
    }

    pub fn rc_data_less_than_sign() -> States {
        States::RcDataLessThanSign(RcDataLessThanSign {})
    }

    pub fn rc_data_end_tag_open() -> States {
        States::RcDataEndTagOpen(RcDataEndTagOpen {})
    }

    pub fn rc_data_end_tag_name(token: token::Token, tmp: String) -> States {
        States::RcDataEndTagName(RcDataEndTagName { token, tmp })
    }

    pub fn raw_text_less_than_sign() -> States {
        States::RawTextLessThanSign(RawTextLessThanSign {})
    }

    pub fn raw_text_end_tag_open() -> States {
        States::RawTextEndTagOpen(RawTextEndTagOpen {})
    }

    pub fn raw_text_end_tag_name() -> States {
        States::RawTextEndTagName(RawTextEndTagName {})
    }

    pub fn script_data_less_than_sign() -> States {
        States::ScriptDataLessThanSign(ScriptDataLessThanSign {})
    }

    pub fn script_data_end_tag_open() -> States {
        States::ScriptDataEndTagOpen(ScriptDataEndTagOpen {})
    }

    pub fn script_data_end_tag_name() -> States {
        States::ScriptDataEndTagName(ScriptDataEndTagName {})
    }

    pub fn script_data_escape_start() -> States {
        States::ScriptDataEscapeStart(ScriptDataEscapeStart {})
    }

    pub fn script_data_escape_start_dash() -> States {
        States::ScriptDataEscapeStartDash(ScriptDataEscapeStartDash {})
    }

    pub fn script_data_escaped() -> States {
        States::ScriptDataEscaped(ScriptDataEscaped {})
    }

    pub fn script_data_escaped_dash() -> States {
        States::ScriptDataEscapedDash(ScriptDataEscapedDash {})
    }

    pub fn script_data_escaped_dash_dash() -> States {
        States::ScriptDataEscapedDashDash(ScriptDataEscapedDashDash {})
    }

    pub fn script_data_escaped_less_than_sign() -> States {
        States::ScriptDataEscapedLessThanSign(ScriptDataEscapedLessThanSign {})
    }

    pub fn script_data_escaped_end_tag_open() -> States {
        States::ScriptDataEscapedEndTagOpen(ScriptDataEscapedEndTagOpen {})
    }

    pub fn script_data_escaped_end_tag_name() -> States {
        States::ScriptDataEscapedEndTagName(ScriptDataEscapedEndTagName {})
    }

    pub fn script_data_double_escape_start() -> States {
        States::ScriptDataDoubleEscapeStart(ScriptDataDoubleEscapeStart {})
    }

    pub fn script_data_double_escaped() -> States {
        States::ScriptDataDoubleEscaped(ScriptDataDoubleEscaped {})
    }

    pub fn script_data_double_escaped_dash() -> States {
        States::ScriptDataDoubleEscapedDash(ScriptDataDoubleEscapedDash {})
    }

    pub fn script_data_double_escaped_dash_dash() -> States {
        States::ScriptDataDoubleEscapedDashDash(ScriptDataDoubleEscapedDashDash {})
    }

    pub fn script_data_double_escaped_less_than_sign() -> States {
        States::ScriptDataDoubleEscapedLessThanSign(ScriptDataDoubleEscapedLessThanSign {})
    }

    pub fn script_data_double_escape_end() -> States {
        States::ScriptDataDoubleEscapeEnd(ScriptDataDoubleEscapeEnd {})
    }

    pub fn before_attribute_name(token: token::Token) -> States {
        States::BeforeAttributeName(BeforeAttributeName { token })
    }

    pub fn attribute_name(token: token::Token) -> States {
        States::AttributeName(AttributeName { token })
    }

    pub fn after_attribute_name(token: token::Token) -> States {
        States::AfterAttributeName(AfterAttributeName { token })
    }

    pub fn before_attribute_value(token: token::Token) -> States {
        States::BeforeAttributeValue(BeforeAttributeValue { token })
    }

    pub fn attribute_value_double_quoted(token: token::Token) -> States {
        States::AttributeValueDoubleQuoted(AttributeValueDoubleQuoted { token })
    }

    pub fn attribute_value_single_quoted(token: token::Token) -> States {
        States::AttributeValueSingleQuoted(AttributeValueSingleQuoted { token })
    }

    pub fn attribute_value_unquoted(token: token::Token) -> States {
        States::AttributeValueUnquoted(AttributeValueUnquoted { token })
    }

    pub fn after_attribute_value_quoted(token: token::Token) -> States {
        States::AfterAttributeValueQuoted(AfterAttributeValueQuoted { token })
    }

    pub fn self_closing_start_tag(token: token::Token) -> States {
        States::SelfClosingStartTag(SelfClosingStartTag { token })
    }

    pub fn bogus_comment(token: token::Token) -> States {
        States::BogusComment(BogusComment { token })
    }

    pub fn markup_declaration_open() -> States {
        States::MarkupDeclarationOpen(MarkupDeclarationOpen {})
    }

    pub fn comment_start(token: token::Token) -> States {
        States::CommentStart(CommentStart { token })
    }

    pub fn comment_start_dash(token: token::Token) -> States {
        States::CommentStartDash(CommentStartDash { token })
    }

    pub fn comment(token: token::Token) -> States {
        States::Comment(Comment { token })
    }

    pub fn comment_less_than_sign(token: token::Token) -> States {
        States::CommentLessThanSign(CommentLessThanSign { token })
    }

    pub fn comment_less_than_sign_bang() -> States {
        States::CommentLessThanSignBang(CommentLessThanSignBang {})
    }

    pub fn comment_less_than_sign_bang_dash() -> States {
        States::CommentLessThanSignBangDash(CommentLessThanSignBangDash {})
    }

    pub fn comment_less_than_sign_bang_dash_dash() -> States {
        States::CommentLessThanSignBangDashDash(CommentLessThanSignBangDashDash {})
    }

    pub fn comment_end_dash(token: token::Token) -> States {
        States::CommentEndDash(CommentEndDash { token })
    }

    pub fn comment_end(token: token::Token) -> States {
        States::CommentEnd(CommentEnd { token })
    }

    pub fn comment_end_bang(token: token::Token) -> States {
        States::CommentEndBang(CommentEndBang { token })
    }

    pub fn doctype() -> States {
        States::Doctype(Doctype {})
    }

    pub fn before_doctype_name() -> States {
        States::BeforeDoctypeName(BeforeDoctypeName {})
    }

    pub fn doctype_name(token: token::Token) -> States {
        States::DoctypeName(DoctypeName { token })
    }

    pub fn after_doctype_name() -> States {
        States::AfterDoctypeName(AfterDoctypeName {})
    }

    pub fn after_doctype_public_keyword() -> States {
        States::AfterDoctypePublicKeyword(AfterDoctypePublicKeyword {})
    }

    pub fn before_doctype_public_identifier() -> States {
        States::BeforeDoctypePublicIdentifier(BeforeDoctypePublicIdentifier {})
    }

    pub fn doctype_public_identifier_double_quoted() -> States {
        States::DoctypePublicIdentifierDoubleQuoted(DoctypePublicIdentifierDoubleQuoted {})
    }

    pub fn doctype_public_identifier_single_quoted() -> States {
        States::DoctypePublicIdentifierSingleQuoted(DoctypePublicIdentifierSingleQuoted {})
    }

    pub fn after_doctype_public_identifier() -> States {
        States::AfterDoctypePublicIdentifier(AfterDoctypePublicIdentifier {})
    }

    pub fn between_doctype_public_and_system_identifiers() -> States {
        States::BetweenDoctypePublicAndSystemIdentifiers(
            BetweenDoctypePublicAndSystemIdentifiers {},
        )
    }

    pub fn after_doctype_system_keyword() -> States {
        States::AfterDoctypeSystemKeyword(AfterDoctypeSystemKeyword {})
    }

    pub fn before_doctype_system_identifier() -> States {
        States::BeforeDoctypeSystemIdentifier(BeforeDoctypeSystemIdentifier {})
    }

    pub fn doctype_system_identifier_double_quoted() -> States {
        States::DoctypeSystemIdentifierDoubleQuoted(DoctypeSystemIdentifierDoubleQuoted {})
    }

    pub fn doctype_system_identifier_single_quoted() -> States {
        States::DoctypeSystemIdentifierSingleQuoted(DoctypeSystemIdentifierSingleQuoted {})
    }

    pub fn after_doctype_system_identifier() -> States {
        States::AfterDoctypeSystemIdentifier(AfterDoctypeSystemIdentifier {})
    }

    pub fn bogus_doctype() -> States {
        States::BogusDoctype(BogusDoctype {})
    }

    pub fn cdata_section() -> States {
        States::CdataSection(CdataSection {})
    }

    pub fn cdata_section_bracket() -> States {
        States::CdataSectionBracket(CdataSectionBracket {})
    }

    pub fn cdata_section_end() -> States {
        States::CdataSectionEnd(CdataSectionEnd {})
    }

    pub fn character_reference(return_state: Box<States>, tmp: String) -> States {
        States::CharacterReference(CharacterReference { return_state, tmp })
    }

    pub fn named_character_reference(return_state: Box<States>, tmp: String) -> States {
        States::NamedCharacterReference(NamedCharacterReference { return_state, tmp })
    }

    pub fn ambiguous_ampersand(return_state: Box<States>) -> States {
        States::AmbiguousAmpersand(AmbiguousAmpersand { return_state })
    }

    pub fn numeric_character_reference(return_state: Box<States>, tmp: String) -> States {
        States::NumericCharacterReference(NumericCharacterReference { return_state, tmp })
    }

    pub fn hexadecimal_character_reference_start(
        return_state: Box<States>,
        tmp: String,
        character_reference_code: CharacterReferenceCode,
    ) -> States {
        States::HexadecimalCharacterReferenceStart(HexadecimalCharacterReferenceStart {
            return_state,
            tmp,
            character_reference_code,
        })
    }

    pub fn decimal_character_reference_start(
        return_state: Box<States>,
        tmp: String,
        character_reference_code: CharacterReferenceCode,
    ) -> States {
        States::DecimalCharacterReferenceStart(DecimalCharacterReferenceStart {
            return_state,
            tmp,
            character_reference_code,
        })
    }

    pub fn hexadecimal_character_reference(
        return_state: Box<States>,
        tmp: String,
        character_reference_code: CharacterReferenceCode,
    ) -> States {
        States::HexadecimalCharacterReference(HexadecimalCharacterReference {
            return_state,
            tmp,
            character_reference_code,
        })
    }

    pub fn decimal_character_reference(
        return_state: Box<States>,
        tmp: String,
        character_reference_code: CharacterReferenceCode,
    ) -> States {
        States::DecimalCharacterReference(DecimalCharacterReference {
            tmp,
            return_state,
            character_reference_code,
        })
    }

    pub fn numeric_character_reference_end(
        return_state: Box<States>,
        tmp: String,
        character_reference_code: CharacterReferenceCode,
    ) -> States {
        States::NumericCharacterReferenceEnd(NumericCharacterReferenceEnd {
            tmp,
            return_state,
            character_reference_code,
        })
    }

    pub fn term() -> States {
        States::Term(Term {})
    }

    // Transitions

    pub fn on_character(self, input: Character) -> TransitionResult {
        use States::*;

        match self {
            Data(state) => state.on_character(input),
            // RcData(state) => state.on_character(input),
            // RawText(state) => state.on_character(input),
            // ScriptData(state) => state.on_character(input),
            // PlainText(state) => state.on_character(input),
            TagOpen(state) => state.on_character(input),
            EndTagOpen(state) => state.on_character(input),
            TagName(state) => state.on_character(input),
            // RcDataLessThanSign(state) => state.on_character(input),
            // RcDataEndTagOpen(state) => state.on_character(input),
            RcDataEndTagName(state) => state.on_character(input),
            // RawTextLessThanSign(state) => state.on_character(input),
            // RawTextEndTagOpen(state) => state.on_character(input),
            // RawTextEndTagName(state) => state.on_character(input),
            // ScriptDataLessThanSign(state) => state.on_character(input),
            // ScriptDataEndTagOpen(state) => state.on_character(input),
            // ScriptDataEndTagName(state) => state.on_character(input),
            // ScriptDataEscapeStart(state) => state.on_character(input),
            // ScriptDataEscapeStartDash(state) => state.on_character(input),
            // ScriptDataEscaped(state) => state.on_character(input),
            // ScriptDataEscapedDash(state) => state.on_character(input),
            // ScriptDataEscapedDashDash(state) => state.on_character(input),
            // ScriptDataEscapedLessThanSign(state) => state.on_character(input),
            // ScriptDataEscapedEndTagOpen(state) => state.on_character(input),
            // ScriptDataEscapedEndTagName(state) => state.on_character(input),
            // ScriptDataDoubleEscapeStart(state) => state.on_character(input),
            // ScriptDataDoubleEscaped(state) => state.on_character(input),
            // ScriptDataDoubleEscapedDash(state) => state.on_character(input),
            // ScriptDataDoubleEscapedDashDash(state) => state.on_character(input),
            // ScriptDataDoubleEscapedLessThanSign(state) => state.on_character(input),
            // ScriptDataDoubleEscapeEnd(state) => state.on_character(input),
            BeforeAttributeName(state) => state.on_character(input),
            AttributeName(state) => state.on_character(input),
            AfterAttributeName(state) => state.on_character(input),
            BeforeAttributeValue(state) => state.on_character(input),
            AttributeValueDoubleQuoted(state) => state.on_character(input),
            AttributeValueSingleQuoted(state) => state.on_character(input),
            AttributeValueUnquoted(state) => state.on_character(input),
            AfterAttributeValueQuoted(state) => state.on_character(input),
            SelfClosingStartTag(state) => state.on_character(input),
            BogusComment(state) => state.on_character(input),
            // MarkupDeclarationOpen(state) => state.on_character(input),
            CommentStart(state) => state.on_character(input),
            // CommentStartDash(state) => state.on_character(input),
            Comment(state) => state.on_character(input),
            // CommentLessThanSign(state) => state.on_character(input),
            // CommentLessThanSignBang(state) => state.on_character(input),
            // CommentLessThanSignBangDash(state) => state.on_character(input),
            // CommentLessThanSignBangDashDash(state) => state.on_character(input),
            CommentEndDash(state) => state.on_character(input),
            CommentEnd(state) => state.on_character(input),
            // CommentEndBang(state) => state.on_character(input),
            Doctype(state) => state.on_character(input),
            BeforeDoctypeName(state) => state.on_character(input),
            DoctypeName(state) => state.on_character(input),
            // AfterDoctypeName(state) => state.on_character(input),
            // AfterDoctypePublicKeyword(state) => state.on_character(input),
            // BeforeDoctypePublicIdentifier(state) => state.on_character(input),
            // DoctypePublicIdentifierDoubleQuoted(state) => state.on_character(input),
            // DoctypePublicIdentifierSingleQuoted(state) => state.on_character(input),
            // AfterDoctypePublicIdentifier(state) => state.on_character(input),
            // BetweenDoctypePublicAndSystemIdentifiers(state) => state.on_character(input),
            // AfterDoctypeSystemKeyword(state) => state.on_character(input),
            // BeforeDoctypeSystemIdentifier(state) => state.on_character(input),
            // DoctypeSystemIdentifierDoubleQuoted(state) => state.on_character(input),
            // DoctypeSystemIdentifierSingleQuoted(state) => state.on_character(input),
            // AfterDoctypeSystemIdentifier(state) => state.on_character(input),
            // BogusDoctype(state) => state.on_character(input),
            // CdataSection(state) => state.on_character(input),
            // CdataSectionBracket(state) => state.on_character(input),
            // CdataSectionEnd(state) => state.on_character(input),
            CharacterReference(state) => state.on_character(input),
            // NamedCharacterReference(state) => state.on_character(input),
            AmbiguousAmpersand(state) => state.on_character(input),
            NumericCharacterReference(state) => state.on_character(input),
            HexadecimalCharacterReferenceStart(state) => state.on_character(input),
            DecimalCharacterReferenceStart(state) => state.on_character(input),
            HexadecimalCharacterReference(state) => state.on_character(input),
            DecimalCharacterReference(state) => state.on_character(input),
            // NumericCharacterReferenceEnd(state) => state.on_character(input),
            _ => Err(errors::StateTransitionError::new(self, "Character")).into(),
        }
    }

    pub fn on_advance(self) -> TransitionResult {
        match self {
            States::NumericCharacterReferenceEnd(state) => state.on_advance(),
            _ => Err(errors::StateTransitionError::new(self, "Advance")).into(),
        }
    }

    pub fn on_next_few_characters(self, input: NextFewCharacters) -> TransitionResult {
        match self {
            States::MarkupDeclarationOpen(state) => state.on_next_few_characters(input),
            _ => Err(errors::StateTransitionError::new(self, "NextFewCharacters")).into(),
        }
    }

    pub fn on_possible_character_reference_with_next_char(
        self,
        input: PossibleCharacterReferenceWithNextChar,
    ) -> TransitionResult {
        match self {
            States::NamedCharacterReference(state) => {
                state.on_possible_character_reference_with_next_char(input)
            }
            _ => Err(errors::StateTransitionError::new(
                self,
                "PossibleCharacterReference",
            ))
            .into(),
        }
    }

    pub fn execute(self, input: StateMachineMessages) -> TransitionResult {
        use StateMachineMessages::*;

        match input {
            Advance => self.on_advance(),
            NextFewCharacters(message) => self.on_next_few_characters(message),
            PossibleCharacterReferenceWithNextChar(message) => {
                self.on_possible_character_reference_with_next_char(message)
            }
            Character(message) => self.on_character(message),
        }
    }

    pub fn into_transition_result(self) -> TransitionResult {
        TransitionResult::from_state(self)
    }
}

impl Default for States {
    fn default() -> Self {
        States::data()
    }
}

#[derive(Clone, Debug, PartialEq, From)]
pub enum StateMachineMessages {
    Advance,
    NextFewCharacters(NextFewCharacters),
    PossibleCharacterReferenceWithNextChar(PossibleCharacterReferenceWithNextChar),
    Character(Character),
}

#[derive(Clone, Copy, Debug, PartialEq, From)]
pub enum Character {
    Char(char),
    LineFeed,
    Null,
    Eof,
}

// Is this just needed for MarkupDeclarationOpen?
#[derive(Clone, Debug, PartialEq, From, AsRef, Deref, DerefMut)]
pub struct NextFewCharacters(Option<String>);

// Is this just needed for NamedCharacterReference?
#[derive(Clone, Debug, PartialEq, From, AsRef)]
pub struct PossibleCharacterReferenceWithNextChar(pub Option<String>, pub Character);
