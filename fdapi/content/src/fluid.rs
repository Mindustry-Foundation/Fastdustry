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