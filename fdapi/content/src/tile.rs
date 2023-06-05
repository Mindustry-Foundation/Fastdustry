use vectora::types::vector::Vector2d;

pub struct Tile {
  pos: Vector2d<i32>
}

impl Tile {
  pub fn pos(&self) -> Vector2d<i32> {
    self.pos
  }
}