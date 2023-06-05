use std::sync::{Mutex, atomic::{AtomicU16, Ordering}};
use once_cell::sync::Lazy;

pub static UNIT_TYPES: Lazy<Mutex<Vec<UnitType>>> = Lazy::new(|| vec![].into());
static NEXT_UNIT_ID: AtomicU16 = AtomicU16::new(0);

pub trait Unit {
  fn r#type(&self) -> UnitType;
}

#[derive(Debug, Default)]
pub struct UnitType {
  id: u16
}

impl UnitType {
  pub fn new() -> Self {
    Self {
      id: 0
    }
  }

  pub fn register(mut self) {
    self.id = NEXT_UNIT_ID.fetch_add(1, Ordering::Relaxed);
    UNIT_TYPES.lock().unwrap().push(self);
  }

  pub fn id(&self) -> u16 {
    self.id
  }
}