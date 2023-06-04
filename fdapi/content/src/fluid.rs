use crate::WithId;

pub trait Fluid {
  fn r#type(&self) -> FluidType;
}

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
  fn new(id: u16) -> Self {
    Self { id }
  }

  fn id(&self) -> u16 {
    self.id
  }
}