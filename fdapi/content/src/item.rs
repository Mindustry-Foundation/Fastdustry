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