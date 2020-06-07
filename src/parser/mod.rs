#![allow(dead_code)]

use std::io;

use log::trace;

use crate::{
    dom::Document,
    tokenizer::{Token, Tokenizer},
};

mod errors;
mod states;
mod transition_result;
mod transitions;
mod force_quirks_check;

use states::States;
use transition_result::TransitionResult;

pub struct Parser<R>
where
    R: io::Read + io::Seek,
{
    document: Document,

    tokenizer: Tokenizer<R>,

    insertion_mode: Option<States>,
    last_token: Option<Token>,

    stack_of_open_elements: Vec<()>,
}

impl<R> Parser<R>
where
    R: io::Read + io::Seek,
{
    pub fn new(r: R) -> Self {
        let document = Document::new();
        let tokenizer = Tokenizer::new(r, false);

        Parser {
            document,
            tokenizer,

            insertion_mode: Some(States::new()),
            last_token: None,
            stack_of_open_elements: vec![],
        }
    }

    pub fn run(&mut self) {
        loop {
            let insertion_mode = self.insertion_mode.take().unwrap();

            trace!("Insertion Mode: {:?}", insertion_mode);
            let res = match insertion_mode {
                States::Term(_) => return,
                _ => {
                    let token = self.tokenizer.next().unwrap();
                    // self.last_token = Some(token);
                    insertion_mode.on_token(token)
                }
            };
            trace!("Result: {:?}", res);
        }
    }
}
