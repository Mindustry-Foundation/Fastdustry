pub trait Floor {
  fn r#type(&self) -> FloorType;
}

#[derive(Debug, Default)]
pub struct FloorType {
  id: u16
}

impl FloorType {
  pub fn new() -> Self {
    Self::default()
  }
}