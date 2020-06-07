use derive_more::{Constructor, From};
use serde::{Deserialize, Serialize};

#[derive(Clone, Constructor, Debug, Default, Deserialize, Eq, From, Hash, PartialEq, Serialize)]
pub struct DocumentType {
    name: String,
    public_id: String,
    system_id: String,
}
