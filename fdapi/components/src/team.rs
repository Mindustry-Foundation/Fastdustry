use palette::Srgba;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone, Copy)]
pub struct Team {
  color: Srgba,
}

impl Team {
  pub fn color(&self) -> &Srgba {
    &self.color
  }

  pub fn color_mut(&mut self) -> &mut Srgba {
    &mut self.color
  }
}
