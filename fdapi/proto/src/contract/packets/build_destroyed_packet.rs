use std::{ convert::TryFrom, io::Error };

use bytebuffer::ByteBuffer;

use super::Packet;

#[derive(Debug, Default)]
pub struct BuildDestroyedPacket {
  building: u32, //координаты, переделать
}

impl Packet for BuildDestroyedPacket {}

impl TryFrom<&Vec<u8>> for BuildDestroyedPacket {
  type Error = Error;
  fn try_from(byte_vector: &Vec<u8>) -> Result<Self, Self::Error> {
    let mut byte_buffer = ByteBuffer::from_bytes(byte_vector);
    let building = byte_buffer.read_u32()?;
    Ok(BuildDestroyedPacket { building })
  }
}

impl Into<Vec<u8>> for BuildDestroyedPacket {
  fn into(self) -> Vec<u8> {
    let mut byte_buffer = ByteBuffer::new();
    byte_buffer.write_u32(self.building);
    byte_buffer.into_vec()
  }
}
