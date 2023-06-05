use std::sync::{Mutex, atomic::{AtomicU16, Ordering}};

use once_cell::sync::Lazy;

pub static TEAMS: Lazy<Mutex<Vec<Team>>> = Lazy::new(|| vec![].into());
static NEXT_TEAM_ID: AtomicU16 = AtomicU16::new(0);

#[derive(Debug, Default)]
pub struct Team {
  id: u16,
}

impl Team {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn register(mut self) {
    self.id = NEXT_TEAM_ID.fetch_add(1, Ordering::Relaxed);
    TEAMS.lock().unwrap().push(self);
  }

  pub fn id(&self) -> u16 {
    self.id
  }
}