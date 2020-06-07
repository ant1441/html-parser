use derive_more::{Constructor, From};
use serde::{Deserialize, Serialize};

#[derive(Clone, Constructor, Debug, Default, Deserialize, Eq, From, Hash, PartialEq, Serialize)]
pub struct ProcessingInstruction {
    data: String,
}

impl ProcessingInstruction {
    pub fn len(&self) -> usize {
        self.data.len()
    }
}
