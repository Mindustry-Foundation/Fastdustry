use palette::Srgba;
use property::Property;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone, Copy, Property)]
#[property(get(public), set(public), mut(disable), ord(desc))]
pub struct Team {
  color: Srgba,
}