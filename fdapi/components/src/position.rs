use serde::{ Deserialize, Serialize };
use nalgebra::Vector2;

#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Position(Vector2<u32>);

#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Velocity(Position);

impl Position {
  pub fn x(&self) -> &u32 {
    &self.0.x
  }

  pub fn y(&self) -> &u32 {
    &self.0.y
  }

  pub fn x_mut(&mut self) -> &mut u32 {
    &mut self.0.x
  }

  pub fn y_mut(&mut self) -> &mut u32 {
    &mut self.0.y
  }
}

impl Velocity {
  pub fn x(&self) -> &u32 {
    self.0.x()
  }

  pub fn y(&self) -> &u32 {
    self.0.y()
  }

  pub fn x_mut(&mut self) -> &mut u32 {
    self.0.x_mut()
  }

  pub fn y_mut(&mut self) -> &mut u32 {
    self.0.y_mut()
  }
}
