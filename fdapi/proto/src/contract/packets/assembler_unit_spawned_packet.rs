use std::{ convert::TryFrom, io::Error };

use bytebuffer::ByteBuffer;
use content::tile::Tile;

use crate::{ WriteStruct, ReadStruct };

use super::Packet;

#[derive(Debug, Default)]
pub struct AssemblerUnitSpawnedPacket {
  tile: Tile,
}

impl Packet for AssemblerUnitSpawnedPacket {}

impl TryFrom<&Vec<u8>> for AssemblerUnitSpawnedPacket {
  type Error = Error;
  fn try_from(byte_vector: &Vec<u8>) -> Result<Self, Self::Error> {
    let mut byte_buffer = ByteBuffer::from_bytes(byte_vector);
    let tile = byte_buffer.read_struct()?;
    Ok(AssemblerUnitSpawnedPacket { tile })
  }
}

impl Into<Vec<u8>> for AssemblerUnitSpawnedPacket {
  fn into(self) -> Vec<u8> {
    let mut byte_buffer = ByteBuffer::new();
    byte_buffer.write_struct(&self.tile);
    byte_buffer.into_vec()
  }
}
