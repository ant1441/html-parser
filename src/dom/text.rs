use std::cell::RefCell;
use std::rc::Rc;

use derive_more::{Deref, DerefMut, From};
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, Default, Deserialize, Eq, From, Hash, PartialEq, Serialize, Deref, DerefMut,
)]
pub struct Text {
    data: String,
}

impl Text {
    pub fn new(data: String) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Text { data }))
    }
}
