use crate::WithId;

pub trait Tile {
  fn r#type(&self) -> TileType;
}

#[derive(Debug, Default)]
pub struct TileType {
  id: u16
}

impl TileType {
  pub fn new() -> Self {
    Self::default()
  }
}

impl WithId for TileType {
  fn new(id: u16) -> Self {
    Self { id }
  }

  fn id(&self) -> u16 {
    self.id
  }
}