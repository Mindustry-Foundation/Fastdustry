use std::{ convert::TryFrom, io::Error };
use bytebuffer::ByteBuffer;
use super::Packet;

#[derive(Debug, Default)]
pub struct BuildingControlSelectPacket { //оба поля id, переделать
  player: u16,
  building: u16,
}

impl Packet for BuildingControlSelectPacket {}

impl TryFrom<&Vec<u8>> for BuildingControlSelectPacket {
  type Error = Error;
  fn try_from(byte_vector: &Vec<u8>) -> Result<Self, Self::Error> {
    let mut byte_buffer = ByteBuffer::from_bytes(byte_vector);
    let player = byte_buffer.read_u16()?;
    let building = byte_buffer.read_u16()?;
    Ok(BuildingControlSelectPacket { player, building })
  }
}

impl Into<Vec<u8>> for BuildingControlSelectPacket {
  fn into(self) -> Vec<u8> {
    let mut byte_buffer = ByteBuffer::new();
    byte_buffer.write_u16(self.player);
    byte_buffer.write_u16(self.building);
    byte_buffer.into_vec()
  }
}
