use std::{ convert::TryFrom, io::Error };
use bytebuffer::ByteBuffer;
use content::{ unit::{ UNIT_TYPES, UnitType }, team::{ Team, TEAMS } };
use crate::{ ReadStruct, WriteStruct };
use super::Packet;

#[derive(Debug)]
pub struct BeginBreakPacket<'a> {
  unit: &'a UnitType, //Unit
  team: &'a Team,
  x: u32,
  y: u32,
}

impl Packet for BeginBreakPacket<'_> {}

impl TryFrom<&Vec<u8>> for BeginBreakPacket<'_> {
  type Error = Error;
  fn try_from(byte_vector: &Vec<u8>) -> Result<Self, Self::Error> {
    let mut byte_buffer = ByteBuffer::from_bytes(byte_vector);
    let unit = byte_buffer.read_struct()?;
    let team = byte_buffer.read_struct()?;
    let x = byte_buffer.read_u32()?;
    let y = byte_buffer.read_u32()?;
    Ok(BeginBreakPacket { unit, team, x, y })
  }
}

impl Into<Vec<u8>> for BeginBreakPacket<'_> {
  fn into(self) -> Vec<u8> {
    let mut byte_buffer = ByteBuffer::new();
    byte_buffer.write_struct(self.unit);
    byte_buffer.write_struct(self.team);
    byte_buffer.write_u32(self.x);
    byte_buffer.write_u32(self.y);
    byte_buffer.into_vec()
  }
}

impl Default for BeginBreakPacket<'_> {
  fn default() -> Self {
    Self {
      unit: UNIT_TYPES.lock().unwrap().get(0).unwrap(),
      team: TEAMS.lock().unwrap().get(0).unwrap(),
      x: 1,
      y: 1,
    }
  }
}
