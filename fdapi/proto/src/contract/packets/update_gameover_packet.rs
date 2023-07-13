use std::{ convert::TryFrom, io::Error };
use bytebuffer::ByteBuffer;
use content::team::{ Team, TEAMS };
use crate::{ ReadStruct, WriteStruct };

use super::Packet;

#[derive(Debug)]
pub struct UpdateGameoverPacket<'a> {
  win_team: &'a Team,
}

impl Packet for UpdateGameoverPacket<'_> {}

impl TryFrom<&Vec<u8>> for UpdateGameoverPacket<'_> {
  type Error = Error;

  fn try_from(byte_vector: &Vec<u8>) -> Result<Self, Self::Error> {
    let mut byte_buffer = ByteBuffer::from_bytes(byte_vector);

    Ok(UpdateGameoverPacket {
      win_team: byte_buffer.read_struct()?,
    })
  }
}

impl Into<Vec<u8>> for UpdateGameoverPacket<'_> {
  fn into(self) -> Vec<u8> {
    let mut byte_buffer = ByteBuffer::new();

    byte_buffer.write_struct(self.win_team);
    byte_buffer.into_vec()
  }
}

impl Default for UpdateGameoverPacket<'_> {
  fn default() -> Self {
    Self {
      // win_team: TEAMS.lock()
      //  .unwrap()
      //  .get(0)
      //.unwrap(),
    }
  }
}
