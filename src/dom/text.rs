use derive_more::{Constructor, From, Deref, DerefMut};
use serde::{Deserialize, Serialize};

#[derive(Clone, Constructor, Debug, Default, Deserialize, Eq, From, Hash, PartialEq, Serialize, Deref, DerefMut)]
pub struct Text {
    data: String,
}
