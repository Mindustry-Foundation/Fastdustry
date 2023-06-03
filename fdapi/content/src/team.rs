use crate::WithId;

#[derive(Debug, Default)]
pub struct Team {
  id: u16,
}

impl Team {
  pub fn new() -> Self {
    Self::default()
  }
}

impl WithId for Team {
  fn id(&self) -> u16 {
    self.id
  }

  fn set_id(&mut self, id: u16) {
    self.id = id;
  }
}