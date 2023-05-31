use once_cell::sync::Lazy;

#[derive(Debug, Default)]
pub struct UnitType {
  id: u16,
}

impl UnitType {
  pub fn new() -> Self {
    Self {
      id: UNIT_TYPES.last()
        .map(|unit_type| unit_type.id)
        .unwrap_or(0),
    }
  }

  pub fn id(&self) -> u16 {
    self.id
  }
}

pub static DAGGER: Lazy<UnitType> = Lazy::new(|| UnitType::new());
pub static MACE: Lazy<UnitType> = Lazy::new(|| UnitType::new());
pub static FORTRESS: Lazy<UnitType> = Lazy::new(|| UnitType::new());
pub static SCEPTER: Lazy<UnitType> = Lazy::new(|| UnitType::new());
pub static REIGN: Lazy<UnitType> = Lazy::new(|| UnitType::new());
pub static NOVA: Lazy<UnitType> = Lazy::new(|| UnitType::new());
pub static PULSAR: Lazy<UnitType> = Lazy::new(|| UnitType::new());
pub static QUASAR: Lazy<UnitType> = Lazy::new(|| UnitType::new());
pub static VELA: Lazy<UnitType> = Lazy::new(|| UnitType::new());
pub static CORVUS: Lazy<UnitType> = Lazy::new(|| UnitType::new());
pub static CRAWLER: Lazy<UnitType> = Lazy::new(|| UnitType::new());
pub static ATRAX: Lazy<UnitType> = Lazy::new(|| UnitType::new());
pub static SPIROCT: Lazy<UnitType> = Lazy::new(|| UnitType::new());
pub static ARKYID: Lazy<UnitType> = Lazy::new(|| UnitType::new());
pub static TOXOPID: Lazy<UnitType> = Lazy::new(|| UnitType::new());
pub static FLARE: Lazy<UnitType> = Lazy::new(|| UnitType::new());
pub static HORIZON: Lazy<UnitType> = Lazy::new(|| UnitType::new());
pub static ZENITH: Lazy<UnitType> = Lazy::new(|| UnitType::new());
pub static ANTUMBRA: Lazy<UnitType> = Lazy::new(|| UnitType::new());
pub static ECLIPSE: Lazy<UnitType> = Lazy::new(|| UnitType::new());
pub static MONO: Lazy<UnitType> = Lazy::new(|| UnitType::new());
pub static POLY: Lazy<UnitType> = Lazy::new(|| UnitType::new());
pub static MEGA: Lazy<UnitType> = Lazy::new(|| UnitType::new());
pub static QUAD: Lazy<UnitType> = Lazy::new(|| UnitType::new());
pub static OCT: Lazy<UnitType> = Lazy::new(|| UnitType::new());
pub static RISSO: Lazy<UnitType> = Lazy::new(|| UnitType::new());
pub static MINKE: Lazy<UnitType> = Lazy::new(|| UnitType::new());
pub static BRYDE: Lazy<UnitType> = Lazy::new(|| UnitType::new());
pub static SEI: Lazy<UnitType> = Lazy::new(|| UnitType::new());
pub static OMURA: Lazy<UnitType> = Lazy::new(|| UnitType::new());
pub static RETUSA: Lazy<UnitType> = Lazy::new(|| UnitType::new());
pub static OXYNOE: Lazy<UnitType> = Lazy::new(|| UnitType::new());
pub static CYERCE: Lazy<UnitType> = Lazy::new(|| UnitType::new());
pub static AEGIRES: Lazy<UnitType> = Lazy::new(|| UnitType::new());
pub static NAVANAX: Lazy<UnitType> = Lazy::new(|| UnitType::new());
pub static ALPHA: Lazy<UnitType> = Lazy::new(|| UnitType::new());
pub static BETA: Lazy<UnitType> = Lazy::new(|| UnitType::new());
pub static GAMMA: Lazy<UnitType> = Lazy::new(|| UnitType::new());
pub static STELL: Lazy<UnitType> = Lazy::new(|| UnitType::new());
pub static LOCUS: Lazy<UnitType> = Lazy::new(|| UnitType::new());
pub static PRECEPT: Lazy<UnitType> = Lazy::new(|| UnitType::new());
pub static VANQUISH: Lazy<UnitType> = Lazy::new(|| UnitType::new());
pub static CONQUER: Lazy<UnitType> = Lazy::new(|| UnitType::new());
pub static MERUI: Lazy<UnitType> = Lazy::new(|| UnitType::new());
pub static CLEROI: Lazy<UnitType> = Lazy::new(|| UnitType::new());
pub static ANTHICUS: Lazy<UnitType> = Lazy::new(|| UnitType::new());
pub static TECTA: Lazy<UnitType> = Lazy::new(|| UnitType::new());
pub static COLLARIS: Lazy<UnitType> = Lazy::new(|| UnitType::new());
pub static ELUDE: Lazy<UnitType> = Lazy::new(|| UnitType::new());
pub static AVERT: Lazy<UnitType> = Lazy::new(|| UnitType::new());
pub static OBVIATE: Lazy<UnitType> = Lazy::new(|| UnitType::new());
pub static QUELL: Lazy<UnitType> = Lazy::new(|| UnitType::new());
pub static DISRUPT: Lazy<UnitType> = Lazy::new(|| UnitType::new());
pub static RENALE: Lazy<UnitType> = Lazy::new(|| UnitType::new());
pub static LATUM: Lazy<UnitType> = Lazy::new(|| UnitType::new());
pub static EVOKE: Lazy<UnitType> = Lazy::new(|| UnitType::new());
pub static INCITE: Lazy<UnitType> = Lazy::new(|| UnitType::new());
pub static EMANATE: Lazy<UnitType> = Lazy::new(|| UnitType::new());
pub static BLOCK: Lazy<UnitType> = Lazy::new(|| UnitType::new());
pub static MANIFOLD: Lazy<UnitType> = Lazy::new(|| UnitType::new());
pub static ASSEMBLY_DRONE: Lazy<UnitType> = Lazy::new(|| UnitType::new());

pub static UNIT_TYPES: Lazy<Vec<&UnitType>> = Lazy::new(||
  vec![
    &DAGGER,
    &MACE,
    &FORTRESS,
    &SCEPTER,
    &REIGN,
    &NOVA,
    &PULSAR,
    &QUASAR,
    &VELA,
    &CORVUS,
    &CRAWLER,
    &ATRAX,
    &SPIROCT,
    &ARKYID,
    &TOXOPID,
    &FLARE,
    &HORIZON,
    &ZENITH,
    &ANTUMBRA,
    &ECLIPSE,
    &MONO,
    &POLY,
    &MEGA,
    &QUAD,
    &OCT,
    &RISSO,
    &MINKE,
    &BRYDE,
    &SEI,
    &OMURA,
    &RETUSA,
    &OXYNOE,
    &CYERCE,
    &AEGIRES,
    &NAVANAX,
    &ALPHA,
    &BETA,
    &GAMMA,
    &STELL,
    &LOCUS,
    &PRECEPT,
    &VANQUISH,
    &CONQUER,
    &MERUI,
    &CLEROI,
    &ANTHICUS,
    &TECTA,
    &COLLARIS,
    &ELUDE,
    &AVERT,
    &OBVIATE,
    &QUELL,
    &DISRUPT,
    &RENALE,
    &LATUM,
    &EVOKE,
    &INCITE,
    &EMANATE,
    &BLOCK,
    &MANIFOLD,
    &ASSEMBLY_DRONE
  ]
);
