use std::ops::RangeInclusive;

use lazy_static::lazy_static;

#[derive(Debug, Default)]
pub struct Team {
  id: u16
}

impl Team {
  pub fn new(id: u16) -> Self {
    Self { id }
  }

  pub fn id(&self) -> u16 {
    self.id
  }
}

lazy_static! {
  pub static ref TEAMS: Vec<Team> = RangeInclusive::new(0, u8::MAX)
    .map(|id| Team::new(id as u16))
    .collect();
}
