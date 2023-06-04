pub trait WithId {
  fn new(id: u16) -> Self;
  fn id(&self) -> u16;
}

#[derive(Debug, Default)]
pub struct Registry<F> where F: WithId + Default {
  entries: Vec<F>
}

impl<F> Registry<F> where F: WithId + Default {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn register(&mut self) -> &F {
    let index = self.entries.len();
    let entry = F::new(index as u16);

    self.entries.push(entry);
    self.entries.get(index).unwrap()
  }
}