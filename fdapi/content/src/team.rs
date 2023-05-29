use std::any::TypeId;
use init_trait::Init;

#[derive(Debug, Default)]
pub struct Team {
  id: TypeId
}

impl Team {
  pub const TEAMS: [Team; 256] = ::init(|id| Team { id });
}