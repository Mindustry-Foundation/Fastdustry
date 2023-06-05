pub trait Block {
  fn r#type(&self) -> BlockType;
}

#[derive(Debug, Default)]
pub struct BlockPlan {}

#[derive(Debug, Default)]
pub struct BlockType {
  id: u16
}

impl BlockType {
  pub fn new() -> Self {
    Self::default()
  }
}