#![allow(dead_code)]

use std::io;

use log::{debug, trace};

use crate::{
    dom::{self, Document},
    tokenizer::{Token, Tokenizer},
};

mod errors;
mod states;
mod transition_result;
mod transitions;

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

    open_elements: Vec<dom::Node>,
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
            open_elements: Vec::new(),
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
            trace!("Document: {:?}", self.document);
            let res = match insertion_mode {
                States::Term(_) => return,
                _ => {
                    let token = if !self.reprocess {
                        self.tokenizer.next().unwrap()
                    } else {
                        self.last_token.take().unwrap()
                    };
                    // self.last_token = Some(token);
                    let ret = insertion_mode.on_token(self, &token);
                    self.last_token = Some(token);
                    ret
                }
            };

            if res.is_err() {
                let next_state_error = res.state().unwrap_err();
                // TODO return err?
                panic!("Parser error: {}", next_state_error);
            }

            self.reprocess = res.reprocess();
            self.insertion_mode = Some(res.state().unwrap());
        }
    }
}
