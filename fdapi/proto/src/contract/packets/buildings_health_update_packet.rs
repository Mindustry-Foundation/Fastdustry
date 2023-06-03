use std::{ convert::TryFrom, io::Error };

use bytebuffer::ByteBuffer;

use super::Packet;

#[derive(Debug, Default)]
pub struct BuildDestroyedPacket {
  buildings: Vec<u32>,
}

impl Packet for BuildDestroyedPacket {}

impl TryFrom<&Vec<u8>> for BuildDestroyedPacket {
  type Error = Error;
  fn try_from(byte_vector: &Vec<u8>) -> Result<Self, Self::Error> {
    let mut byte_buffer = ByteBuffer::from_bytes(byte_vector);
    let mut buildings = Vec::<u32>::new();
    let total_buildings = byte_buffer.read_u32()?;
    for _ in 0..total_buildings {
      buildings.push(byte_buffer.read_u32()?);
    }
    Ok(BuildDestroyedPacket { buildings })
  }
}

impl Into<Vec<u8>> for BuildDestroyedPacket {
  fn into(self) -> Vec<u8> {
    let mut byte_buffer = ByteBuffer::new();
    byte_buffer.write_u32(self.buildings.len() as u32);
    for num in self.buildings {
      byte_buffer.write_u32(num);
    }
    byte_buffer.into_vec()
  }
}
