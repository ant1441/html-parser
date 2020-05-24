use std::io::{self, BufReader, Read, Seek, SeekFrom};
use std::str;

use derive_more::{AsRef, From};
use log::{debug};

mod state_machine;
mod token;
mod transitions;

use state_machine::*;
use token::Token;

pub struct Tokenizer<R>
where
    R: io::Read + io::Seek,
{
    reader: BufReader<R>,
    state_machine: Option<StateMachine>,
}

impl<R> Tokenizer<R>
where
    R: io::Read + io::Seek,
{
    pub fn new(data: R) -> Self {
        Tokenizer {
            reader: BufReader::new(data),
            state_machine: Some(StateMachine::new()),
        }
    }

    fn next_character(&mut self) -> io::Result<Character> {
        let mut potential_char = Vec::with_capacity(4);
        loop {
            if potential_char.len() == 4 {
                panic!("invalid utf-8 in input");
            }
            let mut b = [0; 1];
            match self.reader.read_exact(&mut b) {
                Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                    return Ok(Character::EOF)
                }
                r => r,
            }?;
            if b[0] == 0 {
                return Ok(Character::Null);
            }
            potential_char.push(b[0]);
            match str::from_utf8(&potential_char) {
                Ok("\r") => todo!("handle \\r\\n"),
                Ok("\n") => return Ok(Character::LineFeed),
                Ok(c) => return Ok(c.chars().next().unwrap().into()),
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
        let mut state = self.state_machine.take().unwrap();
        loop {
            let c = self.next_character().unwrap();

            let next_state = state.on_character(c);
            debug!("Next State: {:?}", next_state);
            state = next_state;

            let next_state = match state {
                StateMachine::Term(_) => return,
                StateMachine::Error => panic!("StateMachine Error! {:?}", state),
                StateMachine::MarkupDeclarationOpen(_) => {
                    if self.next_few_characters_are("--") {
                        todo!("MarkupDeclarationOpen::--");
                    } else if self.next_few_characters_are("DOCTYPE") {
                        // TODO: should be case insensitive
                        state.on_next_few_characters(NextFewCharacters(Some("DOCTYPE".to_string())))
                    } else if self.next_few_characters_are("[CDATA[") {
                        todo!("MarkupDeclarationOpen::[CDATA[");
                    } else {
                        todo!("MarkupDeclarationOpen::*");
                    }
                }
                _ => continue,
            };
            debug!("Next State: {:?}", next_state);
            state = next_state;
        }
    }
}

pub(crate) fn emit(token: Token) {
    println!("Emit: {:?}", token);
}

#[derive(Clone, Debug, PartialEq, From)]
pub enum Character {
    Char(char),
    LineFeed,
    Null,
    EOF,
}

// Is this just needed for MarkupDeclarationOpen?
#[derive(Clone, Debug, PartialEq, From, AsRef)]
pub struct NextFewCharacters(Option<String>);
