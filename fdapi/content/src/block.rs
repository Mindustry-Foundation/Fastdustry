use std::fmt::Formatter;

use crate::WithId;

pub trait Block {}

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
  fn id(&self) -> u16 {
    self.id
  }

  fn set_id(&mut self, id: u16) {
    self.id = id;
  }
}

pub struct BlockPlan<'a> {
  pub block: &'a dyn Block,
}

impl<'a> std::fmt::Debug for BlockPlan<'a> {
  fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
    formatter.debug_struct("BlockPlan").finish()
  }
}