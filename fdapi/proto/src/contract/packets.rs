use std::{io::{Error, ErrorKind}, convert::TryFrom};

use bytebuffer::ByteBuffer;
use unique_type_id::UniqueTypeId;

#[derive(UniqueTypeId, Debug)]
#[UniqueTypeIdFile = "fdapi/proto/packets.toml"]
pub struct StreamBegin {
  pub id: u32,
  pub total: u32,
  pub stream_type: u8
}

impl Packet for StreamBegin {}

impl TryFrom<&Vec<u8>> for StreamBegin {
  type Error = Error;

  fn try_from(byte_vector: &Vec<u8>) -> Result<Self, Self::Error> {
    let mut byte_buffer = ByteBuffer::from_bytes(byte_vector);

    match (
      byte_buffer.read_u32(),
      byte_buffer.read_u32(),
      byte_buffer.read_u8()
    ) {
      (Ok(id), Ok(total), Ok(stream_type)) => Ok(StreamBegin {
        id,
        total,
        stream_type
      }),
      _ => Err(Error::new(
        ErrorKind::InvalidData,
        "could not read enough bits from buffer"
      ))
    }
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

#[derive(Debug)]
struct NotEnoughBits;

pub trait Packet : for<'f> TryFrom<&'f Vec<u8>> + Into<Vec<u8>> {}