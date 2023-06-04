use crate::WithId;

pub trait Block {
  fn block_type() -> BlockType;
}

#[derive(Debug, Default)]
pub struct BlockType {
  id: u16
}

impl BlockType {
  pub fn new() -> Self {
    Self::default()
  }
}

impl WithId for BlockType {
  fn new(id: u16) -> Self {
    Self { id }
  }

  fn id(&self) -> u16 {
    self.id
  }
}