use std::{convert::TryFrom, io::Error};
use bytebuffer::ByteBuffer;
use content::team::Team;
use crate::{ReadStruct, WriteStruct};

use super::Packet;

#[derive(Debug, Default)]
pub struct UpdateGameoverPacket {
  win_team: Team
}

impl Packet for UpdateGameoverPacket {}

impl TryFrom<&Vec<u8>> for UpdateGameoverPacket {
  type Error = Error;

  fn try_from(byte_vector: &Vec<u8>) -> Result<Self, Self::Error> {
    let mut byte_buffer = ByteBuffer::from_bytes(byte_vector);

    Ok(UpdateGameoverPacket {
      win_team: byte_buffer.read_struct()?
    })
  }
}

impl Into<Vec<u8>> for UpdateGameoverPacket {
  fn into(self) -> Vec<u8> {
    let mut byte_buffer = ByteBuffer::new();

    byte_buffer.write_struct(self.win_team);
    byte_buffer.into_vec()
  }
}