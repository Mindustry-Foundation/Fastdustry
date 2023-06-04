use crate::WithId;

pub trait Unit {
  fn r#type(&self) -> UnitType;
}

#[derive(Debug, Default)]
pub struct UnitType {
  id: u16
}

impl UnitType {
  pub fn new() -> Self {
    Self::default()
  }
}

impl WithId for UnitType {
  fn new(id: u16) -> Self {
    Self { id }
  }

  fn id(&self) -> u16 {
    self.id
  }
}