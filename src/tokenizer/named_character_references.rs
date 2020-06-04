use std::collections::HashMap;

use once_cell::sync::OnceCell;
use serde::{Serialize, Deserialize};

// TODO gzip this to reduce size
const ENTITIES_JSON: &str = include_str!("entities.json");

type Entities = HashMap<&'static str, Entity>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Entity {
    pub codepoints: Vec<u32>,
    pub characters: String,
}

// static ref GAMES: Vec<Game> = serde_json::from_str(&GAME_JSON).unwrap();
static ENTITIES: OnceCell<Entities> = OnceCell::new();

pub fn get_entities() -> &'static Entities {
    ENTITIES.get_or_init(|| {
        serde_json::from_str(&ENTITIES_JSON).unwrap()
    })
}
