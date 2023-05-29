use std::{ convert::TryFrom, io::Error };

use bytebuffer::ByteBuffer;
use crate::contract::packets::Packet;

#[derive(Debug, Default)]
pub struct StreamBegin {
  pub id: u32,
  pub total: u32,
  pub stream_type: u8,
}

impl Packet for StreamBegin {}

impl TryFrom<&Vec<u8>> for StreamBegin {
  type Error = Error;

  fn try_from(byte_vector: &Vec<u8>) -> Result<Self, Self::Error> {
    let mut byte_buffer = ByteBuffer::from_bytes(byte_vector);

    Ok(StreamBegin {
      id: byte_buffer.read_u32()?,
      total: byte_buffer.read_u32()?,
      stream_type: byte_buffer.read_u8()?,
    })
  }
}

impl Into<Vec<u8>> for StreamBegin {
  fn into(self) -> Vec<u8> {
    let mut byte_buffer = ByteBuffer::new();

    byte_buffer.write_u32(self.id);
    byte_buffer.write_u32(self.total);
    byte_buffer.write_u8(self.stream_type);
    byte_buffer.into_vec()
  }
}
