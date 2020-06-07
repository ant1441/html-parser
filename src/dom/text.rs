use derive_more::{Constructor, From};
use serde::{Deserialize, Serialize};

#[derive(Clone, Constructor, Debug, Default, Deserialize, Eq, From, Hash, PartialEq, Serialize)]
pub struct Text {
    data: String,
}

impl Text {
    pub fn len(&self) -> usize {
        self.data.len()
    }
}
