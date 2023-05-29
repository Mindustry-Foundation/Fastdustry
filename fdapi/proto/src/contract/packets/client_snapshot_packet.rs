use std::{ collections::VecDeque, io::Error, convert::TryFrom };

use bytebuffer::ByteBuffer;
use content::block::BlockPlan;
use vectora::types::vector::Vector2d;

use crate::{ ReadStruct, WriteStruct };
use super::Packet;

#[derive(Debug, Default)]
pub struct ClientSnapshotPacket<'a> {
  pub snapshot_id: u32,
  pub mining_pos: i32,
  pub unit_id: u32,
  pub pos: Vector2d<f32>,
  pub mouse_pos: Vector2d<f32>,
  pub view_pos: Vector2d<f32>,
  pub view_size: Vector2d<f32>,
  pub velocity: Vector2d<f32>,
  pub rotation: f32,
  pub base_rotation: f32,
  pub plans: VecDeque<BlockPlan<'a>>,
  pub dead: bool,
  pub boosting: bool,
  pub shooting: bool,
  pub chatting: bool,
  pub building: bool,
}

impl Packet for ClientSnapshotPacket<'_> {}

impl TryFrom<&Vec<u8>> for ClientSnapshotPacket<'_> {
  type Error = Error;

  fn try_from(byte_vector: &Vec<u8>) -> Result<Self, Self::Error> {
    let mut byte_buffer = ByteBuffer::from_bytes(byte_vector);

    Ok(ClientSnapshotPacket {
      snapshot_id: byte_buffer.read_u32()?,
      unit_id: byte_buffer.read_u32()?,
      dead: byte_buffer.read_u8()? != 0,
      pos: byte_buffer.read_struct()?,
      mouse_pos: byte_buffer.read_struct()?,
      rotation: byte_buffer.read_f32()?,
      base_rotation: byte_buffer.read_f32()?,
      velocity: byte_buffer.read_struct()?,
      mining_pos: byte_buffer.read_i32()?,
      boosting: byte_buffer.read_u8()? != 0,
      shooting: byte_buffer.read_u8()? != 0,
      chatting: byte_buffer.read_u8()? != 0,
      building: byte_buffer.read_u8()? != 0,
      plans: VecDeque::new(),
      view_pos: byte_buffer.read_struct()?,
      view_size: byte_buffer.read_struct()?,
    })
  }
}

impl Into<Vec<u8>> for ClientSnapshotPacket<'_> {
  fn into(self) -> Vec<u8> {
    let mut byte_buffer = ByteBuffer::new();

    byte_buffer.write_u32(self.snapshot_id);
    byte_buffer.write_u32(self.unit_id);
    byte_buffer.write_u8(self.dead as u8);
    byte_buffer.write_struct(&self.pos);
    byte_buffer.write_struct(&self.mouse_pos);
    byte_buffer.write_f32(self.rotation);
    byte_buffer.write_f32(self.base_rotation);
    byte_buffer.write_struct(&self.velocity);
    byte_buffer.write_i32(self.mining_pos);
    byte_buffer.write_u8(self.boosting as u8);
    byte_buffer.write_u8(self.shooting as u8);
    byte_buffer.write_u8(self.chatting as u8);
    byte_buffer.write_u8(self.building as u8);
    // TODO: byte_buffer.write_struct(&self.plans);
    byte_buffer.write_struct(&self.view_pos);
    byte_buffer.write_struct(&self.view_size);
    byte_buffer.into_vec()
  }
}