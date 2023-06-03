use crate::WithId;

#[derive(Debug, Default)]
pub struct FluidType {
  id: u16
}

impl FluidType {
  pub fn new() -> Self {
    Self::default()
  }
}

impl WithId for FluidType {
  fn id(&self) -> u16 {
    self.id
  }

  fn set_id(&mut self, id: u16) {
    self.id = id;
  }
}

pub trait Fluid {}