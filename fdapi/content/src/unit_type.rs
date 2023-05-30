use std::sync::atomic::{AtomicU16, Ordering};
use lazy_static::lazy_static;

static UNIT_TYPE_COUNTER: AtomicU16 = AtomicU16::new(0);

#[derive(Debug, Default)]
pub struct UnitType {
  id: u16,
}

impl UnitType {
  pub fn new() -> Self {
    Self { id: UNIT_TYPE_COUNTER.fetch_add(1, Ordering::SeqCst) }
  }

  pub fn id(&self) -> u16 {
    self.id
  }
}

lazy_static! {
  pub static ref DAGGER: UnitType = UnitType::new();
  pub static ref MACE: UnitType = UnitType::new();
  pub static ref FORTRESS: UnitType = UnitType::new();
  pub static ref SCEPTER: UnitType = UnitType::new();
  pub static ref REIGN: UnitType = UnitType::new();
  pub static ref NOVA: UnitType = UnitType::new();
  pub static ref PULSAR: UnitType = UnitType::new();
  pub static ref QUASAR: UnitType = UnitType::new();
  pub static ref VELA: UnitType = UnitType::new();
  pub static ref CORVUS: UnitType = UnitType::new();
  pub static ref CRAWLER: UnitType = UnitType::new();
  pub static ref ATRAX: UnitType = UnitType::new();
  pub static ref SPIROCT: UnitType = UnitType::new();
  pub static ref ARKYID: UnitType = UnitType::new();
  pub static ref TOXOPID: UnitType = UnitType::new();
  pub static ref FLARE: UnitType = UnitType::new();
  pub static ref HORIZON: UnitType = UnitType::new();
  pub static ref ZENITH: UnitType = UnitType::new();
  pub static ref ANTUMBRA: UnitType = UnitType::new();
  pub static ref ECLIPSE: UnitType = UnitType::new();
  pub static ref MONO: UnitType = UnitType::new();
  pub static ref POLY: UnitType = UnitType::new();
  pub static ref MEGA: UnitType = UnitType::new();
  pub static ref QUAD: UnitType = UnitType::new();
  pub static ref OCT: UnitType = UnitType::new();
  pub static ref RISSO: UnitType = UnitType::new();
  pub static ref MINKE: UnitType = UnitType::new();
  pub static ref BRYDE: UnitType = UnitType::new();
  pub static ref SEI: UnitType = UnitType::new();
  pub static ref OMURA: UnitType = UnitType::new();
  pub static ref RETUSA: UnitType = UnitType::new();
  pub static ref OXYNOE: UnitType = UnitType::new();
  pub static ref CYERCE: UnitType = UnitType::new();
  pub static ref AEGIRES: UnitType = UnitType::new();
  pub static ref NAVANAX: UnitType = UnitType::new();
  pub static ref ALPHA: UnitType = UnitType::new();
  pub static ref BETA: UnitType = UnitType::new();
  pub static ref GAMMA: UnitType = UnitType::new();
  pub static ref STELL: UnitType = UnitType::new();
  pub static ref LOCUS: UnitType = UnitType::new();
  pub static ref PRECEPT: UnitType = UnitType::new();
  pub static ref VANQUISH: UnitType = UnitType::new();
  pub static ref CONQUER: UnitType = UnitType::new();
  pub static ref MERUI: UnitType = UnitType::new();
  pub static ref CLEROI: UnitType = UnitType::new();
  pub static ref ANTHICUS: UnitType = UnitType::new();
  pub static ref TECTA: UnitType = UnitType::new();
  pub static ref COLLARIS: UnitType = UnitType::new();
  pub static ref ELUDE: UnitType = UnitType::new();
  pub static ref AVERT: UnitType = UnitType::new();
  pub static ref OBVIATE: UnitType = UnitType::new();
  pub static ref QUELL: UnitType = UnitType::new();
  pub static ref DISRUPT: UnitType = UnitType::new();
  pub static ref RENALE: UnitType = UnitType::new();
  pub static ref LATUM: UnitType = UnitType::new();
  pub static ref EVOKE: UnitType = UnitType::new();
  pub static ref INCITE: UnitType = UnitType::new();
  pub static ref EMANATE: UnitType = UnitType::new();
  pub static ref BLOCK: UnitType = UnitType::new();
  pub static ref MANIFOLD: UnitType = UnitType::new();
  pub static ref ASSEMBLY_DRONE: UnitType = UnitType::new();
  // pub static ref MISSILE: UnitType = UnitType::new(); // TODO: Я без понятия как это должно работать вообще

  pub static ref UNITS: Vec<UnitType> = {
    Vec::from([])
  };
}