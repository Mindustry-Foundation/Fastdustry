use std::{ convert::TryFrom, io::Error };

use bytebuffer::ByteBuffer;
use vectora::types::vector::Vector2d;

use crate::{ ReadStruct, WriteStruct };

use super::Packet;

#[derive(Debug, Default)]
pub struct AssemblerDroneSpawnedPacket {
  tile: Vector2d<i32>, //Tile
  id: u32,
}

impl Packet for AssemblerDroneSpawnedPacket {}

impl TryFrom<&Vec<u8>> for AssemblerDroneSpawnedPacket {
  type Error = Error;
  fn try_from(byte_vector: &Vec<u8>) -> Result<Self, Self::Error> {
    let mut byte_buffer = ByteBuffer::from_bytes(byte_vector);
    let tile = byte_buffer.read_struct()?;
    let id = byte_buffer.read_u32()?;
    Ok(AssemblerDroneSpawnedPacket { tile, id })
  }
}

impl Into<Vec<u8>> for AssemblerDroneSpawnedPacket {
  fn into(self) -> Vec<u8> {
    let mut byte_buffer = ByteBuffer::new();
    byte_buffer.write_struct(&self.tile);
    byte_buffer.write_u32(self.id);
    byte_buffer.into_vec()
  }
}
