use std::{ ops::Range, convert::TryInto };

use lazy_static::lazy_static;

#[derive(Debug, Default)]
pub struct Team {
  id: u16,
}

impl Team {
  pub fn id(&self) -> u16 {
    self.id
  }
}

lazy_static! {
  pub static ref TEAMS: [Team; 256] = (Range {
    start: 0,
    end: (u8::MAX as u16) + 1,
  })
    .map(|id| Team { id: id as u16 })
    .collect::<Vec<_>>()
    .try_into()
    .unwrap();
}
