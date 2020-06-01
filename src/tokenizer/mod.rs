use std::io::{self, BufReader, Read, Seek, SeekFrom};
use std::{cell::Cell, str};

// use derive_more::{AsRef, From};
use log::debug;

mod errors;
mod states;
mod token;
mod transitions;

use self::{
    states::{Character, States},
    token::Token,
};

pub use errors::{Error, Result, TransitionResult};

pub type Emit = Vec<Token>;

pub struct Tokenizer<R>
where
    R: io::Read + io::Seek,
{
    reader: BufReader<R>,
    state: Option<States>,

    // We collapse multiple Token::Character into Token::Characters
    characters_emit_cache: Cell<Option<Token>>,
}

impl<R> Tokenizer<R>
where
    R: io::Read + io::Seek,
{
    pub fn new(data: R) -> Self {
        Tokenizer {
            reader: BufReader::new(data),
            state: Some(States::new()),

            characters_emit_cache: Cell::new(None),
        }
    }

    fn next_character(&mut self) -> Result<Character> {
        let mut potential_char = Vec::with_capacity(4);
        loop {
            let mut b = [0; 1];
            match self.reader.read_exact(&mut b) {
                Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                    return Ok(Character::Eof)
                }
                r => r,
            }?;
            if b[0] == 0 {
                // TODO: Is this needed? \0 is valid UTF8?
                return Ok(Character::Null);
            }
            potential_char.push(b[0]);
            match str::from_utf8(&potential_char) {
                Ok("\r") => todo!("handle \\r\\n"),
                Ok("\n") => return Ok(Character::LineFeed),
                Ok(c) => return Ok(c.chars().next().unwrap().into()),
                e @ Err(_) if potential_char.len() == 4 => return e.map(|_| unreachable!())?,
                Err(_) => continue,
            }
        }
    }

    fn next_few_characters_are(&mut self, other: &str) -> bool {
        let pos = self.reader.seek(SeekFrom::Current(0)).unwrap();
        let mut buffer = vec![0u8; other.len()];
        if self.reader.read_exact(&mut buffer).is_err() {
            return false;
        }
        match str::from_utf8(&buffer) {
            Ok(s) if s == other => true,
            _ => {
                self.reader.seek(SeekFrom::Current(-(pos as i64))).unwrap();
                false
            }
        }
    }

    pub fn run(&mut self) {
        let mut state = self.state.take().unwrap();
        loop {
            let c = self.next_character().unwrap();

            let res = state.on_character(c);
            state = self.handle_transition_result(res);

            let res = match state {
                States::Term(_) => return,
                States::MarkupDeclarationOpen(_) => {
                    if self.next_few_characters_are("--") {
                        state.on_next_few_characters(Some("--".to_string()).into())
                    } else if self.next_few_characters_are("DOCTYPE") {
                        // TODO: should be case insensitive
                        state.on_next_few_characters(Some("DOCTYPE".to_string()).into())
                    } else if self.next_few_characters_are("[CDATA[") {
                        state.on_next_few_characters(Some("[CDATA[".to_string()).into())
                    } else {
                        todo!("MarkupDeclarationOpen::*");
                    }
                }
                _ => continue,
            };
            state = self.handle_transition_result(res);
        }
    }

    pub fn handle_transition_result(&self, mut res: TransitionResult) -> States {
        for token in res.emits() {
            if !token.is_character() {
                if let Some(cached_token) = self.characters_emit_cache.take() {
                    self.emit(&cached_token);
                }
                self.emit(&token)
            } else if let Some(mut cached_token) = self.characters_emit_cache.take() {
                // Take the cached_token, and add the current char to it
                cached_token.push_token(token);
                self.characters_emit_cache.set(Some(cached_token));
            } else {
                // Make a new Token::Characters, from the current char
                let mut cached_token = Token::Characters(String::new());
                cached_token.push_token(token);
                self.characters_emit_cache.set(Some(cached_token));
            }
        }

        if res.is_err() {
            let next_state_error = res.state().unwrap_err();
            panic!("Tokenizer error: {}", next_state_error);
        }
        let next_state = res.state().unwrap();
        debug!("Next State: {:?}", next_state);
        next_state
    }

    pub fn emit(&self, token: &Token) {
        println!("[EMIT]: {:?}", token);
    }
}
