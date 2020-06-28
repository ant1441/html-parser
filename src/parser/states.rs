use std::io;

use derive_more::{Display, From};

use crate::{
    parser::{errors, Parser, TransitionResult},
    tokenizer::Token,
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
    Initial,
    BeforeHtml,
    BeforeHead,
    InHead,
    InHeadNoscript,
    AfterHead,
    InBody,
    Text,
    InTable,
    InTableText,
    InCaption,
    InColumnGroup,
    InTableBody,
    InRow,
    InCell,
    InSelect,
    InSelectInTable,
    InTemplate,
    AfterBody,
    InFrameset,
    AfterFrameset,
    AfterAfterBody,
    AfterAfterFrameset,

    Term,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct Initial {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct BeforeHtml {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct BeforeHead {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct InHead {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct InHeadNoscript {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct AfterHead {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct InBody {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct Text {
    pub(super) original_insertion_mode: Box<States>,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct InTable {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct InTableText {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct InCaption {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct InColumnGroup {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct InTableBody {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct InRow {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct InCell {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct InSelect {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct InSelectInTable {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct InTemplate {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct AfterBody {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct InFrameset {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct AfterFrameset {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct AfterAfterBody {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct AfterAfterFrameset {}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct Term {}

impl States {
    pub(super) fn new() -> Self {
        States::initial()
    }

    pub(super) fn initial() -> Self {
        States::Initial(Initial {})
    }

    pub(super) fn before_html() -> Self {
        States::BeforeHtml(BeforeHtml {})
    }

    pub(super) fn before_head() -> Self {
        States::BeforeHead(BeforeHead {})
    }

    pub(super) fn in_head() -> Self {
        States::InHead(InHead {})
    }

    pub(super) fn in_head_noscript() -> Self {
        States::InHeadNoscript(InHeadNoscript {})
    }

    pub(super) fn after_head() -> Self {
        States::AfterHead(AfterHead {})
    }

    pub(super) fn in_body() -> Self {
        States::InBody(InBody {})
    }

    pub(super) fn text(original_insertion_mode: Box<States>) -> Self {
        States::Text(Text {
            original_insertion_mode,
        })
    }

    pub(super) fn in_table() -> Self {
        States::InTable(InTable {})
    }

    pub(super) fn in_table_text() -> Self {
        States::InTableText(InTableText {})
    }

    pub(super) fn in_caption() -> Self {
        States::InCaption(InCaption {})
    }

    pub(super) fn in_column_group() -> Self {
        States::InColumnGroup(InColumnGroup {})
    }

    pub(super) fn in_table_body() -> Self {
        States::InTableBody(InTableBody {})
    }

    pub(super) fn in_row() -> Self {
        States::InRow(InRow {})
    }

    pub(super) fn in_cell() -> Self {
        States::InCell(InCell {})
    }

    pub(super) fn in_select() -> Self {
        States::InSelect(InSelect {})
    }

    pub(super) fn in_select_in_table() -> Self {
        States::InSelectInTable(InSelectInTable {})
    }

    pub(super) fn in_template() -> Self {
        States::InTemplate(InTemplate {})
    }

    pub(super) fn after_body() -> Self {
        States::AfterBody(AfterBody {})
    }

    pub(super) fn in_frameset() -> Self {
        States::InFrameset(InFrameset {})
    }

    pub(super) fn after_frameset() -> Self {
        States::AfterFrameset(AfterFrameset {})
    }

    pub(super) fn after_after_body() -> Self {
        States::AfterAfterBody(AfterAfterBody {})
    }

    pub(super) fn after_after_frameset() -> Self {
        States::AfterAfterFrameset(AfterAfterFrameset {})
    }

    pub(super) fn term() -> Self {
        States::Term(Term {})
    }

    // Transitions

    pub(super) fn on_token<R>(self, parser: &mut Parser<R>, input: &Token) -> TransitionResult
    where
        R: io::Read + io::Seek,
    {
        use States::*;

        match self {
            Initial(state) => state.on_token(parser, input),
            BeforeHtml(state) => state.on_token(parser, input),
            BeforeHead(state) => state.on_token(parser, input),
            InHead(state) => state.on_token(parser, input),
            // InHeadNoscript(state) => state.on_token(parser, input),
            AfterHead(state) => state.on_token(parser, input),
            InBody(state) => state.on_token(parser, input),
            Text(state) => state.on_token(parser, input),
            // InTable(state) => state.on_token(parser, input),
            // InTableText(state) => state.on_token(parser, input),
            // InCaption(state) => state.on_token(parser, input),
            // InColumnGroup(state) => state.on_token(parser, input),
            // InTableBody(state) => state.on_token(parser, input),
            // InRow(state) => state.on_token(parser, input),
            // InCell(state) => state.on_token(parser, input),
            // InSelect(state) => state.on_token(parser, input),
            // InSelectInTable(state) => state.on_token(parser, input),
            // InTemplate(state) => state.on_token(parser, input),
            AfterBody(state) => state.on_token(parser, input),
            // InFrameset(state) => state.on_token(parser, input),
            // AfterFrameset(state) => state.on_token(parser, input),
            AfterAfterBody(state) => state.on_token(parser, input),
            // AfterAfterFrameset(state) => state.on_token(parser, input),
            _ => Err(errors::StateTransitionError::new(self, "Token")).into(),
        }
    }

    pub(super) fn execute<R>(
        self,
        parser: &mut Parser<R>,
        input: StateMachineMessages<'_>,
    ) -> TransitionResult
    where
        R: io::Read + io::Seek,
    {
        use StateMachineMessages::*;

        match input {
            Token(token) => self.on_token(parser, token),
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
pub(super) enum StateMachineMessages<'t> {
    Token(&'t Token),
}
