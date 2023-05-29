use std::{ convert::TryFrom, io::Error };

use super::Packet;

#[derive(Debug, Default)]
pub struct WorldDataBeginPacket;

impl Packet for WorldDataBeginPacket {}

impl TryFrom<&Vec<u8>> for WorldDataBeginPacket {
  type Error = Error;
  fn try_from(_: &Vec<u8>) -> Result<Self, Self::Error> {
    Ok(Self::default())
  }
}

impl Into<Vec<u8>> for WorldDataBeginPacket {
  fn into(self) -> Vec<u8> {
    Vec::default()
  }
}
