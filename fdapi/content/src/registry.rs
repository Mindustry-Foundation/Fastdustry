pub trait WithId {
  fn id(&self) -> u16;
  fn set_id(&mut self, id: u16);
}

#[derive(Debug, Default)]
pub struct Registry<F> where F: WithId + Default {
  entries: Vec<F>
}

impl<F> Registry<F> where F: WithId + Default {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn register(&mut self, mut entry: F) {
    entry.set_id(self.entries.len() as u16);

    self.entries.push(entry);
  }
}