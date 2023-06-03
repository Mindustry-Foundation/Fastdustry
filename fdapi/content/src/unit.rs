use crate::WithId;

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
  fn id(&self) -> u16 {
    self.id
  }

  fn set_id(&mut self, id: u16) {
    self.id = id;
  }
}