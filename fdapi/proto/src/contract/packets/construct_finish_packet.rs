use std::{ convert::TryFrom, io::Error };
use bytebuffer::ByteBuffer;
use vectora::types::vector::Vector2d;
use crate::{ ReadStruct, WriteStruct };

use super::Packet;

#[derive(Debug, Default)]
pub struct ConstructFinishPacket {
  pub tile: Vector2d<i32>, //Tile !todo
  pub block: u16, //Block !todo
  pub unit: u16, //Unit !todo
  pub rotation: u8,
  pub team: u16, //Team !todo
  pub config: u16, //BlockConfig !todo
}

impl Packet for ConstructFinishPacket {}

impl TryFrom<&Vec<u8>> for ConstructFinishPacket {
  type Error = Error;
  fn try_from(byte_vector: &Vec<u8>) -> Result<Self, Self::Error> {
    let mut byte_buffer = ByteBuffer::from_bytes(byte_vector);
    let tile = byte_buffer.read_struct()?;
    let block = byte_buffer.read_u16()?;
    let unit = byte_buffer.read_u16()?;
    let rotation = byte_buffer.read_u8()?;
    let team = byte_buffer.read_u16()?;
    let config = byte_buffer.read_u16()?;
    Ok(ConstructFinishPacket {
      tile,
      block,
      unit,
      rotation,
      team,
      config,
    })
  }
}

impl Into<Vec<u8>> for ConstructFinishPacket {
  fn into(self) -> Vec<u8> {
    let mut byte_buffer = ByteBuffer::new();
    byte_buffer.write_struct(&self.tile);
    byte_buffer.write_u16(self.block);
    byte_buffer.write_u16(self.unit);
    byte_buffer.write_u8(self.rotation);
    byte_buffer.write_u16(self.team);
    byte_buffer.write_u16(self.config);
    byte_buffer.into_vec()
  }
}
