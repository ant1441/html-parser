use derive_more::{AsRef, Deref, DerefMut, Display, From, Into};

use crate::tokenizer::{
    errors,
    token::{StartTag, Token},
    Codepoint, TransitionResult,
};

macro_rules! create_states {
    ($($s:ident,)+) => {
        // #[derive(Copy, Clone, Debug, PartialEq, Eq, Display)]
         #[derive(Debug, PartialEq, Eq, Display)]
        pub(super) enum States {
            $(
                $s($s),
            )*
        }

        $(
            impl ::std::fmt::Display for $s {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
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
pub(super) struct Data {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct RcData {
    pub(crate) tmp: String,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct RawText {
    pub(crate) tmp: String,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct ScriptData {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct PlainText {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct TagOpen {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct EndTagOpen {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct TagName {
    pub(crate) token: Token,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct RcDataLessThanSign {
    pub(crate) tmp: String,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct RcDataEndTagOpen {
    pub(crate) tmp: String,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct RcDataEndTagName {
    pub(crate) token: Token,
    pub(crate) tmp: String,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct RawTextLessThanSign {
    pub(crate) tmp: String,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct RawTextEndTagOpen {
    pub(crate) tmp: String,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct RawTextEndTagName {
    pub(crate) token: Token,
    pub(crate) tmp: String,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct ScriptDataLessThanSign {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct ScriptDataEndTagOpen {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct ScriptDataEndTagName {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct ScriptDataEscapeStart {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct ScriptDataEscapeStartDash {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct ScriptDataEscaped {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct ScriptDataEscapedDash {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct ScriptDataEscapedDashDash {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct ScriptDataEscapedLessThanSign {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct ScriptDataEscapedEndTagOpen {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct ScriptDataEscapedEndTagName {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct ScriptDataDoubleEscapeStart {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct ScriptDataDoubleEscaped {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct ScriptDataDoubleEscapedDash {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct ScriptDataDoubleEscapedDashDash {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct ScriptDataDoubleEscapedLessThanSign {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct ScriptDataDoubleEscapeEnd {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct BeforeAttributeName {
    pub(crate) token: Token,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct AttributeName {
    pub(crate) token: Token,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct AfterAttributeName {
    pub(crate) token: Token,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct BeforeAttributeValue {
    pub(crate) token: Token,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct AttributeValueDoubleQuoted {
    pub(crate) token: Token,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct AttributeValueSingleQuoted {
    pub(crate) token: Token,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct AttributeValueUnquoted {
    pub(crate) token: Token,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct AfterAttributeValueQuoted {
    pub(crate) token: Token,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct SelfClosingStartTag {
    pub(crate) token: Token,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct BogusComment {
    pub(crate) token: Token,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct MarkupDeclarationOpen {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct CommentStart {
    pub(crate) token: Token,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct CommentStartDash {
    pub(crate) token: Token,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct Comment {
    pub(crate) token: Token,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct CommentLessThanSign {
    pub(crate) token: Token,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct CommentLessThanSignBang {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct CommentLessThanSignBangDash {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct CommentLessThanSignBangDashDash {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct CommentEndDash {
    pub(crate) token: Token,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct CommentEnd {
    pub(crate) token: Token,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct CommentEndBang {
    pub(crate) token: Token,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct Doctype {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct BeforeDoctypeName {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct DoctypeName {
    pub(crate) token: Token,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct AfterDoctypeName {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct AfterDoctypePublicKeyword {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct BeforeDoctypePublicIdentifier {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct DoctypePublicIdentifierDoubleQuoted {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct DoctypePublicIdentifierSingleQuoted {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct AfterDoctypePublicIdentifier {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct BetweenDoctypePublicAndSystemIdentifiers {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct AfterDoctypeSystemKeyword {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct BeforeDoctypeSystemIdentifier {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct DoctypeSystemIdentifierDoubleQuoted {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct DoctypeSystemIdentifierSingleQuoted {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct AfterDoctypeSystemIdentifier {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct BogusDoctype {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct CdataSection {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct CdataSectionBracket {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct CdataSectionEnd {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct CharacterReference {
    pub(crate) return_state: Box<States>,
    pub(crate) tmp: String,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct NamedCharacterReference {
    pub(crate) return_state: Box<States>,
    pub(crate) tmp: String,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct AmbiguousAmpersand {
    pub(crate) return_state: Box<States>,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct NumericCharacterReference {
    pub(crate) return_state: Box<States>,
    pub(crate) tmp: String,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct HexadecimalCharacterReferenceStart {
    pub(crate) tmp: String,
    pub(crate) return_state: Box<States>,
    pub(crate) character_reference_code: CharacterReferenceCode,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct DecimalCharacterReferenceStart {
    pub(crate) tmp: String,
    pub(crate) return_state: Box<States>,
    pub(crate) character_reference_code: CharacterReferenceCode,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct HexadecimalCharacterReference {
    pub(crate) tmp: String,
    pub(crate) return_state: Box<States>,
    pub(crate) character_reference_code: CharacterReferenceCode,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct DecimalCharacterReference {
    pub(crate) tmp: String,
    pub(crate) return_state: Box<States>,
    pub(crate) character_reference_code: CharacterReferenceCode,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct NumericCharacterReferenceEnd {
    pub(crate) tmp: String,
    pub(crate) return_state: Box<States>,
    pub(crate) character_reference_code: CharacterReferenceCode,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct Term {}

pub(crate) type CharacterReferenceCode = Codepoint;

impl States {
    pub(super) fn new() -> Self {
        States::data()
    }

    pub(super) fn data() -> Self {
        States::Data(Data {})
    }

    pub(super) fn rc_data(tmp: String) -> Self {
        States::RcData(RcData { tmp })
    }

    pub(super) fn raw_text(tmp: String) -> Self {
        States::RawText(RawText { tmp })
    }

    pub(super) fn script_data() -> Self {
        States::ScriptData(ScriptData {})
    }

    pub(super) fn plain_text() -> Self {
        States::PlainText(PlainText {})
    }

    pub(super) fn tag_open() -> Self {
        States::TagOpen(TagOpen {})
    }

    pub(super) fn end_tag_open() -> Self {
        States::EndTagOpen(EndTagOpen {})
    }

    pub(super) fn tag_name<T: Into<Token>>(token: T) -> Self {
        States::TagName(TagName {
            token: token.into(),
        })
    }

    pub(super) fn rc_data_less_than_sign(tmp: String) -> Self {
        States::RcDataLessThanSign(RcDataLessThanSign { tmp })
    }

    pub(super) fn rc_data_end_tag_open(tmp: String) -> Self {
        States::RcDataEndTagOpen(RcDataEndTagOpen { tmp })
    }

    pub(super) fn rc_data_end_tag_name<T: Into<Token>>(token: T, tmp: String) -> Self {
        States::RcDataEndTagName(RcDataEndTagName {
            token: token.into(),
            tmp,
        })
    }

    pub(super) fn raw_text_less_than_sign(tmp: String) -> Self {
        States::RawTextLessThanSign(RawTextLessThanSign { tmp })
    }

    pub(super) fn raw_text_end_tag_open(tmp: String) -> Self {
        States::RawTextEndTagOpen(RawTextEndTagOpen { tmp })
    }

    pub(super) fn raw_text_end_tag_name<T: Into<Token>>(token: T, tmp: String) -> Self {
        States::RawTextEndTagName(RawTextEndTagName {
            token: token.into(),
            tmp,
        })
    }

    pub(super) fn script_data_less_than_sign() -> Self {
        States::ScriptDataLessThanSign(ScriptDataLessThanSign {})
    }

    pub(super) fn script_data_end_tag_open() -> Self {
        States::ScriptDataEndTagOpen(ScriptDataEndTagOpen {})
    }

    pub(super) fn script_data_end_tag_name() -> Self {
        States::ScriptDataEndTagName(ScriptDataEndTagName {})
    }

    pub(super) fn script_data_escape_start() -> Self {
        States::ScriptDataEscapeStart(ScriptDataEscapeStart {})
    }

    pub(super) fn script_data_escape_start_dash() -> Self {
        States::ScriptDataEscapeStartDash(ScriptDataEscapeStartDash {})
    }

    pub(super) fn script_data_escaped() -> Self {
        States::ScriptDataEscaped(ScriptDataEscaped {})
    }

    pub(super) fn script_data_escaped_dash() -> Self {
        States::ScriptDataEscapedDash(ScriptDataEscapedDash {})
    }

    pub(super) fn script_data_escaped_dash_dash() -> Self {
        States::ScriptDataEscapedDashDash(ScriptDataEscapedDashDash {})
    }

    pub(super) fn script_data_escaped_less_than_sign() -> Self {
        States::ScriptDataEscapedLessThanSign(ScriptDataEscapedLessThanSign {})
    }

    pub(super) fn script_data_escaped_end_tag_open() -> Self {
        States::ScriptDataEscapedEndTagOpen(ScriptDataEscapedEndTagOpen {})
    }

    pub(super) fn script_data_escaped_end_tag_name() -> Self {
        States::ScriptDataEscapedEndTagName(ScriptDataEscapedEndTagName {})
    }

    pub(super) fn script_data_double_escape_start() -> Self {
        States::ScriptDataDoubleEscapeStart(ScriptDataDoubleEscapeStart {})
    }

    pub(super) fn script_data_double_escaped() -> Self {
        States::ScriptDataDoubleEscaped(ScriptDataDoubleEscaped {})
    }

    pub(super) fn script_data_double_escaped_dash() -> Self {
        States::ScriptDataDoubleEscapedDash(ScriptDataDoubleEscapedDash {})
    }

    pub(super) fn script_data_double_escaped_dash_dash() -> Self {
        States::ScriptDataDoubleEscapedDashDash(ScriptDataDoubleEscapedDashDash {})
    }

    pub(super) fn script_data_double_escaped_less_than_sign() -> Self {
        States::ScriptDataDoubleEscapedLessThanSign(ScriptDataDoubleEscapedLessThanSign {})
    }

    pub(super) fn script_data_double_escape_end() -> Self {
        States::ScriptDataDoubleEscapeEnd(ScriptDataDoubleEscapeEnd {})
    }

    pub(super) fn before_attribute_name<T: Into<Token>>(token: T) -> Self {
        States::BeforeAttributeName(BeforeAttributeName {
            token: token.into(),
        })
    }

    pub(super) fn attribute_name<T: Into<Token>>(token: T) -> Self {
        States::AttributeName(AttributeName {
            token: token.into(),
        })
    }

    pub(super) fn after_attribute_name<T: Into<Token>>(token: T) -> Self {
        States::AfterAttributeName(AfterAttributeName {
            token: token.into(),
        })
    }

    pub(super) fn before_attribute_value<T: Into<Token>>(token: T) -> Self {
        States::BeforeAttributeValue(BeforeAttributeValue {
            token: token.into(),
        })
    }

    pub(super) fn attribute_value_double_quoted<T: Into<Token>>(token: T) -> Self {
        States::AttributeValueDoubleQuoted(AttributeValueDoubleQuoted {
            token: token.into(),
        })
    }

    pub(super) fn attribute_value_single_quoted<T: Into<Token>>(token: T) -> Self {
        States::AttributeValueSingleQuoted(AttributeValueSingleQuoted {
            token: token.into(),
        })
    }

    pub(super) fn attribute_value_unquoted<T: Into<Token>>(token: T) -> Self {
        States::AttributeValueUnquoted(AttributeValueUnquoted {
            token: token.into(),
        })
    }

    pub(super) fn after_attribute_value_quoted<T: Into<Token>>(token: T) -> Self {
        States::AfterAttributeValueQuoted(AfterAttributeValueQuoted {
            token: token.into(),
        })
    }

    pub(super) fn self_closing_start_tag<T: Into<Token>>(token: T) -> Self {
        States::SelfClosingStartTag(SelfClosingStartTag {
            token: token.into(),
        })
    }

    pub(super) fn bogus_comment<T: Into<Token>>(token: T) -> Self {
        States::BogusComment(BogusComment {
            token: token.into(),
        })
    }

    pub(super) fn markup_declaration_open() -> Self {
        States::MarkupDeclarationOpen(MarkupDeclarationOpen {})
    }

    pub(super) fn comment_start<T: Into<Token>>(token: T) -> Self {
        States::CommentStart(CommentStart {
            token: token.into(),
        })
    }

    pub(super) fn comment_start_dash<T: Into<Token>>(token: T) -> Self {
        States::CommentStartDash(CommentStartDash {
            token: token.into(),
        })
    }

    pub(super) fn comment<T: Into<Token>>(token: T) -> Self {
        States::Comment(Comment {
            token: token.into(),
        })
    }

    pub(super) fn comment_less_than_sign<T: Into<Token>>(token: T) -> Self {
        States::CommentLessThanSign(CommentLessThanSign {
            token: token.into(),
        })
    }

    pub(super) fn comment_less_than_sign_bang() -> Self {
        States::CommentLessThanSignBang(CommentLessThanSignBang {})
    }

    pub(super) fn comment_less_than_sign_bang_dash() -> Self {
        States::CommentLessThanSignBangDash(CommentLessThanSignBangDash {})
    }

    pub(super) fn comment_less_than_sign_bang_dash_dash() -> Self {
        States::CommentLessThanSignBangDashDash(CommentLessThanSignBangDashDash {})
    }

    pub(super) fn comment_end_dash<T: Into<Token>>(token: T) -> Self {
        States::CommentEndDash(CommentEndDash {
            token: token.into(),
        })
    }

    pub(super) fn comment_end<T: Into<Token>>(token: T) -> Self {
        States::CommentEnd(CommentEnd {
            token: token.into(),
        })
    }

    pub(super) fn comment_end_bang<T: Into<Token>>(token: T) -> Self {
        States::CommentEndBang(CommentEndBang {
            token: token.into(),
        })
    }

    pub(super) fn doctype() -> Self {
        States::Doctype(Doctype {})
    }

    pub(super) fn before_doctype_name() -> Self {
        States::BeforeDoctypeName(BeforeDoctypeName {})
    }

    pub(super) fn doctype_name<T: Into<Token>>(token: T) -> Self {
        States::DoctypeName(DoctypeName {
            token: token.into(),
        })
    }

    pub(super) fn after_doctype_name() -> Self {
        States::AfterDoctypeName(AfterDoctypeName {})
    }

    pub(super) fn after_doctype_public_keyword() -> Self {
        States::AfterDoctypePublicKeyword(AfterDoctypePublicKeyword {})
    }

    pub(super) fn before_doctype_public_identifier() -> Self {
        States::BeforeDoctypePublicIdentifier(BeforeDoctypePublicIdentifier {})
    }

    pub(super) fn doctype_public_identifier_double_quoted() -> Self {
        States::DoctypePublicIdentifierDoubleQuoted(DoctypePublicIdentifierDoubleQuoted {})
    }

    pub(super) fn doctype_public_identifier_single_quoted() -> Self {
        States::DoctypePublicIdentifierSingleQuoted(DoctypePublicIdentifierSingleQuoted {})
    }

    pub(super) fn after_doctype_public_identifier() -> Self {
        States::AfterDoctypePublicIdentifier(AfterDoctypePublicIdentifier {})
    }

    pub(super) fn between_doctype_public_and_system_identifiers() -> Self {
        States::BetweenDoctypePublicAndSystemIdentifiers(
            BetweenDoctypePublicAndSystemIdentifiers {},
        )
    }

    pub(super) fn after_doctype_system_keyword() -> Self {
        States::AfterDoctypeSystemKeyword(AfterDoctypeSystemKeyword {})
    }

    pub(super) fn before_doctype_system_identifier() -> Self {
        States::BeforeDoctypeSystemIdentifier(BeforeDoctypeSystemIdentifier {})
    }

    pub(super) fn doctype_system_identifier_double_quoted() -> Self {
        States::DoctypeSystemIdentifierDoubleQuoted(DoctypeSystemIdentifierDoubleQuoted {})
    }

    pub(super) fn doctype_system_identifier_single_quoted() -> Self {
        States::DoctypeSystemIdentifierSingleQuoted(DoctypeSystemIdentifierSingleQuoted {})
    }

    pub(super) fn after_doctype_system_identifier() -> Self {
        States::AfterDoctypeSystemIdentifier(AfterDoctypeSystemIdentifier {})
    }

    pub(super) fn bogus_doctype() -> Self {
        States::BogusDoctype(BogusDoctype {})
    }

    pub(super) fn cdata_section() -> Self {
        States::CdataSection(CdataSection {})
    }

    pub(super) fn cdata_section_bracket() -> Self {
        States::CdataSectionBracket(CdataSectionBracket {})
    }

    pub(super) fn cdata_section_end() -> Self {
        States::CdataSectionEnd(CdataSectionEnd {})
    }

    pub(super) fn character_reference<S: Into<States>>(return_state: S, tmp: String) -> Self {
        States::CharacterReference(CharacterReference {
            return_state: Box::new(return_state.into()),
            tmp,
        })
    }

    pub(super) fn named_character_reference(return_state: Box<States>, tmp: String) -> Self {
        States::NamedCharacterReference(NamedCharacterReference { return_state, tmp })
    }

    pub(super) fn ambiguous_ampersand(return_state: Box<States>) -> Self {
        States::AmbiguousAmpersand(AmbiguousAmpersand { return_state })
    }

    pub(super) fn numeric_character_reference(return_state: Box<States>, tmp: String) -> Self {
        States::NumericCharacterReference(NumericCharacterReference { return_state, tmp })
    }

    pub(super) fn hexadecimal_character_reference_start(
        return_state: Box<States>,
        tmp: String,
        character_reference_code: CharacterReferenceCode,
    ) -> Self {
        States::HexadecimalCharacterReferenceStart(HexadecimalCharacterReferenceStart {
            return_state,
            tmp,
            character_reference_code,
        })
    }

    pub(super) fn decimal_character_reference_start(
        return_state: Box<States>,
        tmp: String,
        character_reference_code: CharacterReferenceCode,
    ) -> Self {
        States::DecimalCharacterReferenceStart(DecimalCharacterReferenceStart {
            return_state,
            tmp,
            character_reference_code,
        })
    }

    pub(super) fn hexadecimal_character_reference(
        return_state: Box<States>,
        tmp: String,
        character_reference_code: CharacterReferenceCode,
    ) -> Self {
        States::HexadecimalCharacterReference(HexadecimalCharacterReference {
            return_state,
            tmp,
            character_reference_code,
        })
    }

    pub(super) fn decimal_character_reference(
        return_state: Box<States>,
        tmp: String,
        character_reference_code: CharacterReferenceCode,
    ) -> Self {
        States::DecimalCharacterReference(DecimalCharacterReference {
            tmp,
            return_state,
            character_reference_code,
        })
    }

    pub(super) fn numeric_character_reference_end(
        return_state: Box<States>,
        tmp: String,
        character_reference_code: CharacterReferenceCode,
    ) -> Self {
        States::NumericCharacterReferenceEnd(NumericCharacterReferenceEnd {
            tmp,
            return_state,
            character_reference_code,
        })
    }

    pub(super) fn term() -> Self {
        States::Term(Term {})
    }

    // Transitions

    pub(super) fn on_character(self, input: Character) -> TransitionResult {
        match self {
            States::Data(state) => state.on_character(input),
            States::RcData(state) => state.on_character(input),
            States::RawText(state) => state.on_character(input),
            States::ScriptData(state) => state.on_character(input),
            States::PlainText(state) => state.on_character(input),
            States::TagOpen(state) => state.on_character(input),
            States::EndTagOpen(state) => state.on_character(input),
            States::TagName(state) => state.on_character(input),
            States::RcDataLessThanSign(state) => state.on_character(input),
            States::RcDataEndTagOpen(state) => state.on_character(input),
            // States::RcDataEndTagName(state) => (see on_character_and_last_start_tag)
            States::RawTextLessThanSign(state) => state.on_character(input),
            States::RawTextEndTagOpen(state) => state.on_character(input),
            // States::RawTextEndTagName(state) => (see on_character_and_last_start_tag)
            // States::ScriptDataLessThanSign(state) => state.on_character(input),
            // States::ScriptDataEndTagOpen(state) => state.on_character(input),
            // States::ScriptDataEndTagName(state) => (see on_character_and_last_start_tag)
            // States::ScriptDataEscapeStart(state) => state.on_character(input),
            // States::ScriptDataEscapeStartDash(state) => state.on_character(input),
            // States::ScriptDataEscaped(state) => state.on_character(input),
            // States::ScriptDataEscapedDash(state) => state.on_character(input),
            // States::ScriptDataEscapedDashDash(state) => state.on_character(input),
            // States::ScriptDataEscapedLessThanSign(state) => state.on_character(input),
            // States::ScriptDataEscapedEndTagOpen(state) => state.on_character(input),
            // States::ScriptDataEscapedEndTagName(state) => (see on_character_and_last_start_tag)
            // States::ScriptDataDoubleEscapeStart(state) => state.on_character(input),
            // States::ScriptDataDoubleEscaped(state) => state.on_character(input),
            // States::ScriptDataDoubleEscapedDash(state) => state.on_character(input),
            // States::ScriptDataDoubleEscapedDashDash(state) => state.on_character(input),
            // States::ScriptDataDoubleEscapedLessThanSign(state) => state.on_character(input),
            // States::ScriptDataDoubleEscapeEnd(state) => state.on_character(input),
            States::BeforeAttributeName(state) => state.on_character(input),
            States::AttributeName(state) => state.on_character(input),
            States::AfterAttributeName(state) => state.on_character(input),
            States::BeforeAttributeValue(state) => state.on_character(input),
            States::AttributeValueDoubleQuoted(state) => state.on_character(input),
            States::AttributeValueSingleQuoted(state) => state.on_character(input),
            States::AttributeValueUnquoted(state) => state.on_character(input),
            States::AfterAttributeValueQuoted(state) => state.on_character(input),
            States::SelfClosingStartTag(state) => state.on_character(input),
            States::BogusComment(state) => state.on_character(input),
            // States::MarkupDeclarationOpen (see on_next_few_characters)
            States::CommentStart(state) => state.on_character(input),
            States::CommentStartDash(state) => state.on_character(input),
            States::Comment(state) => state.on_character(input),
            // States::CommentLessThanSign(state) => state.on_character(input),
            // States::CommentLessThanSignBang(state) => state.on_character(input),
            // States::CommentLessThanSignBangDash(state) => state.on_character(input),
            // States::CommentLessThanSignBangDashDash(state) => state.on_character(input),
            States::CommentEndDash(state) => state.on_character(input),
            States::CommentEnd(state) => state.on_character(input),
            States::CommentEndBang(state) => state.on_character(input),
            States::Doctype(state) => state.on_character(input),
            States::BeforeDoctypeName(state) => state.on_character(input),
            States::DoctypeName(state) => state.on_character(input),
            // States::AfterDoctypeName(state) => state.on_character(input),
            // States::AfterDoctypePublicKeyword(state) => state.on_character(input),
            // States::BeforeDoctypePublicIdentifier(state) => state.on_character(input),
            // States::DoctypePublicIdentifierDoubleQuoted(state) => state.on_character(input),
            // States::DoctypePublicIdentifierSingleQuoted(state) => state.on_character(input),
            // States::AfterDoctypePublicIdentifier(state) => state.on_character(input),
            // States::BetweenDoctypePublicAndSystemIdentifiers(state) => state.on_character(input),
            // States::AfterDoctypeSystemKeyword(state) => state.on_character(input),
            // States::BeforeDoctypeSystemIdentifier(state) => state.on_character(input),
            // States::DoctypeSystemIdentifierDoubleQuoted(state) => state.on_character(input),
            // States::DoctypeSystemIdentifierSingleQuoted(state) => state.on_character(input),
            // States::AfterDoctypeSystemIdentifier(state) => state.on_character(input),
            // States::BogusDoctype(state) => state.on_character(input),
            // States::CdataSection(state) => state.on_character(input),
            // States::CdataSectionBracket(state) => state.on_character(input),
            // States::CdataSectionEnd(state) => state.on_character(input),
            States::CharacterReference(state) => state.on_character(input),
            // States::NamedCharacterReference (see on_possible_character_reference_with_next_char)
            States::AmbiguousAmpersand(state) => state.on_character(input),
            States::NumericCharacterReference(state) => state.on_character(input),
            States::HexadecimalCharacterReferenceStart(state) => state.on_character(input),
            States::DecimalCharacterReferenceStart(state) => state.on_character(input),
            States::HexadecimalCharacterReference(state) => state.on_character(input),
            States::DecimalCharacterReference(state) => state.on_character(input),
            // States::NumericCharacterReferenceEnd (see on_advance)
            _ => Err(errors::StateTransitionError::new(self, "Character")).into(),
        }
    }

    pub(super) fn on_character_and_last_start_tag(
        self,
        input: CharacterAndLastStartTag,
    ) -> TransitionResult {
        match self {
            States::RcDataEndTagName(state) => state.on_character_and_last_start_tag(input),
            States::RawTextEndTagName(state) => state.on_character_and_last_start_tag(input),
            // States::ScriptDataEndTagName(state) => state.on_character_and_last_start_tag(input),
            // States::ScriptDataEscapedEndTagName(state) => state.on_character_and_last_start_tag(input),
            _ => Err(errors::StateTransitionError::new(
                self,
                "CharacterAndLastStartTag",
            ))
            .into(),
        }
    }

    pub(super) fn on_advance(self) -> TransitionResult {
        match self {
            States::NumericCharacterReferenceEnd(state) => state.on_advance(),
            _ => Err(errors::StateTransitionError::new(self, "Advance")).into(),
        }
    }

    pub(super) fn on_next_few_characters(self, input: &NextFewCharacters) -> TransitionResult {
        match self {
            States::MarkupDeclarationOpen(state) => state.on_next_few_characters(input),
            _ => Err(errors::StateTransitionError::new(self, "NextFewCharacters")).into(),
        }
    }

    pub(super) fn on_possible_character_reference_with_next_char(
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

    pub(super) fn execute(self, input: StateMachineMessages) -> TransitionResult {
        match input {
            StateMachineMessages::Advance => self.on_advance(),
            StateMachineMessages::NextFewCharacters(message) => {
                self.on_next_few_characters(&message)
            }
            StateMachineMessages::PossibleCharacterReferenceWithNextChar(message) => {
                self.on_possible_character_reference_with_next_char(message)
            }
            StateMachineMessages::Character(message) => self.on_character(message),
            StateMachineMessages::CharacterAndLastStartTag(message) => {
                self.on_character_and_last_start_tag(message)
            }
        }
    }

    pub(super) fn into_transition_result(self) -> TransitionResult {
        TransitionResult::from_state(self)
    }
}

impl Default for States {
    fn default() -> Self {
        States::new()
    }
}

#[derive(Clone, Debug, PartialEq, From)]
pub(super) enum StateMachineMessages {
    Advance,
    NextFewCharacters(NextFewCharacters),
    PossibleCharacterReferenceWithNextChar(PossibleCharacterReferenceWithNextChar),
    Character(Character),
    CharacterAndLastStartTag(CharacterAndLastStartTag),
}

#[derive(Clone, Copy, Debug, PartialEq, From)]
pub(super) enum Character {
    Char(char),
    Eof,
}

#[derive(Clone, Debug, PartialEq, From, Into)]
pub(super) struct CharacterAndLastStartTag(Character, Option<StartTag>);

// Is this just needed for MarkupDeclarationOpen?
#[derive(Clone, Debug, PartialEq, From, Into, AsRef, Deref, DerefMut)]
pub(super) struct NextFewCharacters(Option<String>);

// Is this just needed for NamedCharacterReference?
#[derive(Clone, Debug, PartialEq, From, Into, AsRef)]
pub(super) struct PossibleCharacterReferenceWithNextChar(Option<String>, Character);
