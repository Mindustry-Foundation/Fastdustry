use std::{io::Error, convert::TryFrom};
use bytebuffer::ByteBuffer;
use crate::{ReadStruct, WriteStruct};
use super::Packet;

#[derive(Debug, Default)]
struct WarningToastPacket {
  glyph: u32,
  text: String
}

impl Packet for WarningToastPacket {}

impl TryFrom<&Vec<u8>> for WarningToastPacket {
  type Error = Error;

  fn try_from(byte_vector: &Vec<u8>) -> Result<Self, Self::Error> {
    let mut byte_buffer = ByteBuffer::from_bytes(byte_vector);

    Ok(WarningToastPacket {
      glyph: byte_buffer.read_u32()?,
      text: byte_buffer.read_struct()?
    })
  }
}

impl Into<Vec<u8>> for WarningToastPacket {
  fn into(self) -> Vec<u8> {
    let mut byte_buffer = ByteBuffer::new();

    byte_buffer.write_u32(self.glyph);
    byte_buffer.write_struct(&self.text);
    byte_buffer.into_vec()
  }
}