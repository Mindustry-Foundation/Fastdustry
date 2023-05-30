use lazy_static::lazy_static;

#[derive(Debug, Default)]
pub struct UnitType {
  id: u16,
}

impl UnitType {
  pub fn new() -> Self {
    Self {
      id: UNITS.last()
        .map(|unit_type| unit_type.id)
        .unwrap_or(0),
    }
  }

  pub fn id(&self) -> u16 {
    self.id
  }
}

lazy_static! {
  static ref DAGGER: UnitType = UnitType::new();
  static ref MACE: UnitType = UnitType::new();
  static ref FORTRESS: UnitType = UnitType::new();
  static ref SCEPTER: UnitType = UnitType::new();
  static ref REIGN: UnitType = UnitType::new();
  static ref NOVA: UnitType = UnitType::new();
  static ref PULSAR: UnitType = UnitType::new();
  static ref QUASAR: UnitType = UnitType::new();
  static ref VELA: UnitType = UnitType::new();
  static ref CORVUS: UnitType = UnitType::new();
  static ref CRAWLER: UnitType = UnitType::new();
  static ref ATRAX: UnitType = UnitType::new();
  static ref SPIROCT: UnitType = UnitType::new();
  static ref ARKYID: UnitType = UnitType::new();
  static ref TOXOPID: UnitType = UnitType::new();
  static ref FLARE: UnitType = UnitType::new();
  static ref HORIZON: UnitType = UnitType::new();
  static ref ZENITH: UnitType = UnitType::new();
  static ref ANTUMBRA: UnitType = UnitType::new();
  static ref ECLIPSE: UnitType = UnitType::new();
  static ref MONO: UnitType = UnitType::new();
  static ref POLY: UnitType = UnitType::new();
  static ref MEGA: UnitType = UnitType::new();
  static ref QUAD: UnitType = UnitType::new();
  static ref OCT: UnitType = UnitType::new();
  static ref RISSO: UnitType = UnitType::new();
  static ref MINKE: UnitType = UnitType::new();
  static ref BRYDE: UnitType = UnitType::new();
  static ref SEI: UnitType = UnitType::new();
  static ref OMURA: UnitType = UnitType::new();
  static ref RETUSA: UnitType = UnitType::new();
  static ref OXYNOE: UnitType = UnitType::new();
  static ref CYERCE: UnitType = UnitType::new();
  static ref AEGIRES: UnitType = UnitType::new();
  static ref NAVANAX: UnitType = UnitType::new();
  static ref ALPHA: UnitType = UnitType::new();
  static ref BETA: UnitType = UnitType::new();
  static ref GAMMA: UnitType = UnitType::new();
  static ref STELL: UnitType = UnitType::new();
  static ref LOCUS: UnitType = UnitType::new();
  static ref PRECEPT: UnitType = UnitType::new();
  static ref VANQUISH: UnitType = UnitType::new();
  static ref CONQUER: UnitType = UnitType::new();
  static ref MERUI: UnitType = UnitType::new();
  static ref CLEROI: UnitType = UnitType::new();
  static ref ANTHICUS: UnitType = UnitType::new();
  static ref TECTA: UnitType = UnitType::new();
  static ref COLLARIS: UnitType = UnitType::new();
  static ref ELUDE: UnitType = UnitType::new();
  static ref AVERT: UnitType = UnitType::new();
  static ref OBVIATE: UnitType = UnitType::new();
  static ref QUELL: UnitType = UnitType::new();
  static ref DISRUPT: UnitType = UnitType::new();
  static ref RENALE: UnitType = UnitType::new();
  static ref LATUM: UnitType = UnitType::new();
  static ref EVOKE: UnitType = UnitType::new();
  static ref INCITE: UnitType = UnitType::new();
  static ref EMANATE: UnitType = UnitType::new();
  static ref BLOCK: UnitType = UnitType::new();
  static ref MANIFOLD: UnitType = UnitType::new();
  static ref ASSEMBLY_DRONE: UnitType = UnitType::new();

  pub static ref UNITS: Vec<UnitType> = vec![];
}
