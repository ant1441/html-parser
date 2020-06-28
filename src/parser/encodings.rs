// The file has a structure of
// [type]
// type:
//   encodings: [Encoding]
//   heading: str
//
// We want to convert this to just [Encoding]

use once_cell::sync::OnceCell;
use serde::Deserialize;

// TODO gzip this to reduce size
const ENCODINGS_JSON: &str = include_str!("encodings.json");

type FileEncodings = Vec<FileSection>;

#[derive(Deserialize, Debug)]
struct FileSection {
    encodings: Vec<FileEncoding>,
    heading: String,
}

#[derive(Deserialize, Debug)]
struct FileEncoding {
    labels: Vec<String>,
    name: String,
}

pub type Encodings = Vec<Encoding>;

#[derive(Deserialize, Debug)]
pub struct Encoding {
    pub labels: Vec<String>,
    pub name: String,
    pub heading: String,
}

fn from_file_encodings(file_encodings: FileEncodings) -> Encodings {
    let mut encodings: Encodings = Vec::new();
    for file_section in file_encodings {
        for encoding in file_section.encodings {
            let encoding = Encoding {
                labels: encoding.labels,
                name: encoding.name,
                heading: file_section.heading.clone(),
            };
            encodings.push(encoding);
        }
    }
    encodings
}

// static ref GAMES: Vec<Game> = serde_json::from_str(&GAME_JSON).unwrap();
static ENCODINGS: OnceCell<Encodings> = OnceCell::new();

pub fn get() -> &'static Encodings {
    ENCODINGS.get_or_init(|| {
        let file_encodings = serde_json::from_str(&ENCODINGS_JSON).unwrap();
        from_file_encodings(file_encodings)
    })
}

#[must_use]
pub fn get_encoding(name: &str) -> Option<&'static Encoding> {
    let encodings = get();
    encodings.iter().find(|e| {
        e.labels
            .iter()
            .any(|l| l.eq_ignore_ascii_case(&name.trim()))
    })
}
