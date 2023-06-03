use crate::WithId;

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
  fn id(&self) -> u16 {
    self.id
  }

  fn set_id(&mut self, id: u16) {
    self.id = id;
  }
}

pub trait Item {}