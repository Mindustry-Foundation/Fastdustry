use std::{ convert::TryFrom, io::Error };
use bytebuffer::ByteBuffer;
use super::Packet;

#[derive(Debug, Default)]
pub struct ClearItemsPacket {
  building: u16, //id, переделать
}

impl Packet for ClearItemsPacket {}

impl TryFrom<&Vec<u8>> for ClearItemsPacket {
  type Error = Error;
  fn try_from(byte_vector: &Vec<u8>) -> Result<Self, Self::Error> {
    let mut byte_buffer = ByteBuffer::from_bytes(byte_vector);
    let building = byte_buffer.read_u16()?;
    Ok(ClearItemsPacket { building })
  }
}

impl Into<Vec<u8>> for ClearItemsPacket {
  fn into(self) -> Vec<u8> {
    let mut byte_buffer = ByteBuffer::new();
    byte_buffer.write_u16(self.building);
    byte_buffer.into_vec()
  }
}
