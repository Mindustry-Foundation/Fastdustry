use std::{ convert::TryFrom, io::Error };
use bytebuffer::ByteBuffer;
use crate::{ contract::packets::Packet, ReadStruct, WriteStruct };

#[derive(Debug, Default)]
pub struct AnnouncePacket {
  message: String,
}

impl Packet for AnnouncePacket {}

impl TryFrom<&Vec<u8>> for AnnouncePacket {
  type Error = Error;
  fn try_from(byte_vector: &Vec<u8>) -> Result<Self, Self::Error> {
    let mut byte_buffer = ByteBuffer::from_bytes(byte_vector);
    let message = byte_buffer.read_struct()?;
    Ok(AnnouncePacket { message })
  }
}

impl Into<Vec<u8>> for AnnouncePacket {
  fn into(self) -> Vec<u8> {
    let mut byte_buffer = ByteBuffer::new();
    byte_buffer.write_struct(&self.message);
    byte_buffer.into_vec()
  }
}
