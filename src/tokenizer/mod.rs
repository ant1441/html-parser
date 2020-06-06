use std::io::{self, BufReader, Read, Seek, SeekFrom};
use std::{cell::Cell, str};

// use derive_more::{AsRef, From};
use log::{debug, trace};

mod codepoint;
mod errors;
mod named_character_references;
mod states;
mod token;
mod transitions;

use self::{
    states::{Character, NamedCharacterReference, PossibleCharacterReferenceWithNextChar, States},
    token::Token,
};

pub use codepoint::Codepoint;
pub use errors::{Error, Result, TransitionResult};
pub use named_character_references::get_entities;

const USE_EMIT_CACHE: bool = true;

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
            // TODO, we assume this is UTF-8
            // To be standard compliant we should use the
            // [encoding sniffing algorithm](https://html.spec.whatwg.org/multipage/parsing.html#encoding-sniffing-algorithm)
            reader: BufReader::new(data),
            state: Some(States::new()),

            characters_emit_cache: Cell::new(None),
        }
    }

    fn peek_next_character(&mut self) -> Result<Character> {
        let pos = self.reader.seek(SeekFrom::Current(0)).unwrap();
        let ret = self.next_character()?;
        trace!("Peeked char: {:?}", ret);
        self.reader.seek(SeekFrom::Start(pos)).unwrap();
        Ok(ret)
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
            let c = match str::from_utf8(&potential_char) {
                Ok("\r") => todo!("handle \\r\\n"),
                Ok("\n") => Ok(Character::LineFeed),
                Ok(c) => Ok(c.chars().next().unwrap().into()),
                e @ Err(_) if potential_char.len() == 4 => {
                    debug!("Invalid UTF8: {:x?}", potential_char);
                    e.map(|_| unreachable!())?
                }
                Err(_) => continue,
            };
            trace!("Read character: {:?}", c);
            return c;
        }
    }

    fn next_few_characters_are(&mut self, other: &str, case_insesitive: bool) -> bool {
        let pos = self.reader.seek(SeekFrom::Current(0)).unwrap();
        let mut buffer = vec![0u8; other.len()];
        if self.reader.read_exact(&mut buffer).is_err() {
            return false;
        }
        let read = str::from_utf8(&buffer);
        trace!("next_few_characters_are:: Read {:?}", read);
        match read {
            Ok(s)
                if (!case_insesitive && (s == other))
                    || (case_insesitive && (s.eq_ignore_ascii_case(other))) =>
            {
                true
            }
            _ => {
                self.reader.seek(SeekFrom::Start(pos)).unwrap();
                false
            }
        }
    }

    // Consume the maximum number of characters possible,
    // where the consumed characters are identical to one of the identifiers in the first column of the named character references table.
    // Append each character to the temporary buffer when it's consumed.
    fn find_named_character_reference(
        &mut self,
        original_character: Character,
        tmp: &mut String,
    ) -> Result<Option<String>> {
        let entities = get_entities();
        let identifiers = entities.keys().collect::<std::collections::HashSet<_>>();
        trace!(
            "Searching for named character referece ({} names)",
            identifiers.len()
        );
        let original_char = match original_character {
            // Fairly sure this has to be a Char('&')
            Character::Char(c) => c,
            Character::LineFeed => '\n',
            Character::Eof | Character::Null => {
                todo!("find_named_character_reference: How to handle EOF/NULL?");
            }
        };
        tmp.push(original_char);

        let mut found_ident = None;
        let mut last_valid_reader_pos = self.reader.seek(SeekFrom::Current(0)).unwrap();

        loop {
            trace!("Checking {:?} against idents", tmp);
            // ownership woes
            let r_tmp = tmp.clone();
            if let Some(&&ident) = identifiers.iter().find(|&&ident| ident.starts_with(&r_tmp)) {
                if ident == tmp {
                    found_ident = Some(ident.to_string());
                    last_valid_reader_pos = self.reader.seek(SeekFrom::Current(0)).unwrap();
                    trace!(
                        "Exact character reference match found at pos: {:?}",
                        last_valid_reader_pos
                    );
                }
                // We have an initial match, read next char from file and check again
                let next_char = match self.next_character()? {
                    Character::Eof => {
                        todo!("find_named_character_reference: How to handle EOF/NULL?")
                    }
                    Character::LineFeed => '\n',
                    Character::Null => '\0',
                    Character::Char(c) => c,
                };
                tmp.push(next_char);
            } else {
                trace!("{:?} didn't match the start of any idents", tmp);

                // If we ever found anything, we should seek back 1 char
                if found_ident.is_some() {
                    // Remove the excess char we read in (remember, not only a single byte!)
                    tmp.pop();
                    self.reader
                        .seek(SeekFrom::Start(last_valid_reader_pos))
                        .unwrap();
                }
                break;
            }
        }

        Ok(found_ident)
    }

    pub fn run(&mut self) {
        // IDEAS:
        // * Reconsume via a flag on TransitionResult
        //   ```
        //   if res.reconsume: goto just after self.next_character
        //
        // TODO:
        // '<' in Script tag...
        // StartTag(StartTag { name: "t.length;r++)console.log(\"actionqueue\",c(t[r]))}function&&&&&&&&&&&&&&&",
        let mut state = self.state.take().unwrap();

        let mut c = self.next_character().unwrap();
        // we set reconsume to true as we've already read the initial char
        let mut reconsume = true;
        loop {
            trace!("State ({}): {:?}", if reconsume { "R" } else { "-" }, state);
            let res = match state {
                States::Term(_) => return,
                States::MarkupDeclarationOpen(ref m) => {
                    if self.next_few_characters_are("--", false) {
                        state.on_next_few_characters(Some("--".to_string()).into())
                    } else if self.next_few_characters_are("DOCTYPE", true) {
                        state.on_next_few_characters(Some("DOCTYPE".to_string()).into())
                    } else if self.next_few_characters_are("[CDATA[", false) {
                        state.on_next_few_characters(Some("[CDATA[".to_string()).into())
                    } else {
                        todo!("MarkupDeclarationOpen::{:?}", m);
                    }
                }
                States::NamedCharacterReference(NamedCharacterReference {
                    mut tmp,
                    return_state,
                }) => {
                    let possible_char_ref =
                        self.find_named_character_reference(c, &mut tmp).unwrap();
                    let reconstructed_state =
                        States::NamedCharacterReference(NamedCharacterReference {
                            tmp,
                            return_state,
                        });
                    let next_char = self.peek_next_character().unwrap();
                    reconstructed_state.on_possible_character_reference_with_next_char(
                        PossibleCharacterReferenceWithNextChar(possible_char_ref, next_char),
                    )
                }

                States::NumericCharacterReferenceEnd(_) => state.on_advance(),
                _ => {
                    if !reconsume {
                        // TODO: Tokenizer internal state
                        c = self.next_character().unwrap();
                    }
                    state.on_character(c)
                }
            };
            let ret = self.handle_transition_result(res);
            // TODO: Tokenizer internal state
            state = ret.0;
            // TODO: Tokenizer internal state
            reconsume = ret.1;
        }
    }

    pub fn handle_transition_result(&self, mut res: TransitionResult) -> (States, bool) {
        for token in res.emits() {
            if USE_EMIT_CACHE {
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
            } else {
                self.emit(&token)
            }
        }

        if res.is_err() {
            let next_state_error = res.state().unwrap_err();
            panic!("Tokenizer error: {}", next_state_error);
        }
        let reconsume = res.reconsume();
        let next_state = res.state().unwrap();
        debug!("Next State: {:?}", next_state);
        (next_state, reconsume)
    }

    pub fn emit(&self, token: &Token) {
        println!("[EMIT]: {}", token);
    }
}
