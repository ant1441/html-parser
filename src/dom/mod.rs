#![allow(dead_code)]

use derive_more::{Constructor};

#[derive(Constructor)]
pub struct DocumentType {
  name: String,
  public_id: String,
  system_id: String,
}
