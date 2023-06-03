use crate::WithId;
#[derive(Debug, Default)]
pub struct FloorType {
  id: u16
}

impl FloorType {
  pub fn new() -> Self {
    Self::default()
  }
}

impl WithId for FloorType {
  fn id(&self) -> u16 {
    self.id
  }

  fn set_id(&mut self, id: u16) {
    self.id = id;
  }
}