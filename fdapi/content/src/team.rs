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
  fn new(id: u16) -> Self {
    Self { id }
  }

  fn id(&self) -> u16 {
    self.id
  }
}