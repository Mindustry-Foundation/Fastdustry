use std::{ convert::TryFrom, io::Error };

use bytebuffer::ByteBuffer;
use crate::contract::packets::Packet;

#[derive(Debug, Default)]
pub struct BlockSnapshotPacket {
  pub total: u16,
  pub data: Vec<u8>,
}

impl Packet for BlockSnapshotPacket {}

impl TryFrom<&Vec<u8>> for BlockSnapshotPacket {
  type Error = Error;

  fn try_from(byte_vector: &Vec<u8>) -> Result<Self, Self::Error> {
    let mut byte_buffer = ByteBuffer::from_bytes(byte_vector);
    let total = byte_buffer.read_u16()?;
    let data = byte_buffer.read_bytes(total as usize)?;
    Ok(BlockSnapshotPacket { total, data })
  }
}

impl Into<Vec<u8>> for BlockSnapshotPacket {
  fn into(self) -> Vec<u8> {
    let mut byte_buffer = ByteBuffer::new();
    byte_buffer.write_u16(self.total);
    byte_buffer.write_bytes(&self.data);
    byte_buffer.into_vec()
  }
}
