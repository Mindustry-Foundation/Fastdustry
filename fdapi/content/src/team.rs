use std::ops::RangeInclusive;

use once_cell::sync::Lazy;

#[derive(Debug, Default)]
pub struct Team {
  id: u16,
}

impl Team {
  pub fn new(id: u16) -> Self {
    Self { id }
  }

  pub fn id(&self) -> u16 {
    self.id
  }
}

pub static TEAMS: Lazy<Vec<Team>> = Lazy::new(||
  RangeInclusive::new(0, u8::MAX)
    .map(|id| Team::new(id as u16))
    .collect()
);
