use std::{ convert::TryFrom, io::Error };

use bytebuffer::ByteBuffer;
use super::Packet;

#[derive(Debug, Default)]
pub struct StreamChunk {
  pub id: u32,
  pub data: Vec<u8>,
}

impl Packet for StreamChunk {}

impl TryFrom<&Vec<u8>> for StreamChunk {
  type Error = Error;

  fn try_from(byte_vector: &Vec<u8>) -> Result<Self, Self::Error> {
    let mut byte_buffer = ByteBuffer::from_bytes(byte_vector);
    let id = byte_buffer.read_u32()?;
    let data = byte_buffer.read_u32().map(|length| byte_buffer.read_bytes(length as usize))??;

    Ok(StreamChunk { id, data })
  }
}

impl Into<Vec<u8>> for StreamChunk {
  fn into(self) -> Vec<u8> {
    let mut byte_buffer = ByteBuffer::new();
    byte_buffer.write_u32(self.id);
    byte_buffer.write_bytes(&self.data);
    byte_buffer.into_vec()
  }
}
