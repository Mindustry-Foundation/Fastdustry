use std::{ convert::TryFrom, io::Error };
use bytebuffer::ByteBuffer;
use super::Packet;

#[derive(Debug, Default)]
pub struct ConnectConfirmPacket {}

impl Packet for ConnectConfirmPacket {}

impl TryFrom<&Vec<u8>> for ConnectConfirmPacket {
  type Error = Error;
  fn try_from(byte_vector: &Vec<u8>) -> Result<Self, Self::Error> {
    let mut byte_buffer = ByteBuffer::from_bytes(byte_vector);
    Ok(ConnectConfirmPacket {})
  }
}

impl Into<Vec<u8>> for ConnectConfirmPacket {
  fn into(self) -> Vec<u8> {
    let mut byte_buffer = ByteBuffer::new();
    byte_buffer.into_vec()
  }
}