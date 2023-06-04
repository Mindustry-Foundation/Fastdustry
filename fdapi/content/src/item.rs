use crate::WithId;

pub trait Item {
  fn r#type(&self) -> ItemType;
}

#[derive(Debug, Default)]
pub struct ItemType {
  id: u16
}

impl ItemType {
  pub fn new() -> Self {
    Self::default()
  }
}

impl WithId for ItemType {
  fn new(id: u16) -> Self {
    Self { id }
  }

  fn id(&self) -> u16 {
    self.id
  }
}