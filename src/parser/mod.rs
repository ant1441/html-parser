#![allow(dead_code)]

use std::io;

use log::{debug, trace};

use crate::{
    dom::{self, Document},
    tokenizer::{TagName, Token, Tokenizer},
};

mod errors;
mod open_elements_stack;
mod states;
mod transition_result;
mod transitions;

use open_elements_stack::OpenElementsStack;
use states::States;
use transition_result::TransitionResult;

pub struct Parser<R>
where
    R: io::Read + io::Seek,
{
    document: Document,

    tokenizer: Tokenizer<R>,

    insertion_mode: Option<States>,
    reprocess: bool,
    last_token: Option<Token>,

    open_elements: OpenElementsStack,
}

impl<R> Parser<R>
where
    R: io::Read + io::Seek,
{
    pub fn new(r: R) -> Self {
        let document: Document = Default::default();
        let tokenizer = Tokenizer::new(r, false);

        Parser {
            document,
            tokenizer,

            insertion_mode: Some(States::new()),
            reprocess: false,
            last_token: None,
            open_elements: Default::default(),
        }
    }

    pub fn run(&mut self) {
        loop {
            let insertion_mode = self.insertion_mode.take().unwrap();

            debug!(
                "State ({}): {:?}",
                if self.reprocess { "R" } else { "-" },
                insertion_mode
            );
            let res = match insertion_mode {
                States::Term(_) => return,
                _ => {
                    let token = if !self.reprocess {
                        self.tokenizer.next().unwrap()
                    } else {
                        self.last_token.take().unwrap()
                    };

                    // tree construction dispatcher
                    if self.is_tree_construction_first_case(&token) {
                        trace!("Received token {:?}", token);
                        let ret = insertion_mode.on_token(self, &token);
                        self.last_token = Some(token);
                        ret
                    } else {
                        todo!("Parser: Process the token according to the rules given in the section for parsing tokens in foreign content.");
                    }
                }
            };

            if res.is_err() {
                let next_state_error = res.state().unwrap_err();
                // TODO return err?
                panic!("Parser error: {}", next_state_error);
            }

            trace!("Document: {:?}", self.document);
            self.reprocess = res.reprocess();
            self.insertion_mode = Some(res.state().unwrap());
        }
    }

    // https://html.spec.whatwg.org/multipage/parsing.html#tree-construction
    fn is_tree_construction_first_case(&self, token: &Token) -> bool {
        // If the stack of open elements is empty
        if self.open_elements.is_empty() {
            return true;
        }

        let adjusted_current_node = self.adjusted_current_node().expect(
            "No adjusted_current_node when there are elements on the stack of open elements",
        );
        let is_token_start_tag = token.is_start_tag();
        let is_token_character = token.is_start_tag();
        let is_token_eof = token.is_eof();
        let token_tag_name = token.tag_name();

        // If the adjusted current node is an element in the HTML namespace
        if adjusted_current_node.namespace() == dom::Namespace::HTML {
            return true;
        }

        // If the adjusted current node is a MathML text integration point and
        // the token is a start tag whose tag name is neither "mglyph" nor "malignmark"
        if adjusted_current_node.is_mathml_text_integration_point()
            && is_token_start_tag
            && (token_tag_name != Some(&TagName::Mglyph)
                && token_tag_name != Some(&TagName::Malignmark))
        {
            return true;
        }

        // If the adjusted current node is a MathML text integration point and
        // the token is a character token
        if adjusted_current_node.is_mathml_text_integration_point() && is_token_character {
            return true;
        }

        // If the adjusted current node is a MathML annotation-xml element and
        // the token is a start tag whose tag name is "svg"
        if adjusted_current_node.name() == &TagName::AnnotationXml
            && is_token_start_tag
            && token_tag_name == Some(&TagName::Svg)
        {
            return true;
        }

        // If the adjusted current node is an HTML integration point and
        // the token is a start tag
        if adjusted_current_node.is_html_integration_point() && is_token_start_tag {
            return true;
        }

        // If the adjusted current node is an HTML integration point and
        // the token is a character token
        if adjusted_current_node.is_html_integration_point() && is_token_character {
            return true;
        }

        // If the token is an end-of-file token
        if is_token_eof {
            return true;
        }

        false
    }

    pub fn adjusted_current_node(&self) -> Option<&dom::Element> {
        if
        /* self.parsing_algorithm == HTMLFragmentParsing && self.open_elements.len() == 1 */
        false {
            todo!("Parser::adjusted_current_node with HTMLFragmentParsing");
        } else {
            self.open_elements.last()
        }
    }
}
