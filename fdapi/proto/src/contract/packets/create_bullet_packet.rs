use std::{ convert::TryFrom, io::Error };
use bytebuffer::ByteBuffer;
use super::Packet;

#[derive(Debug, Default)]
pub struct CreateBulletPacket {
  pub bullet_type: u16, //BulletType !todo
  pub team: u16, //Team !todo
  pub x: f32,
  pub y: f32,
  pub angle: f32,
  pub damage: f32,
  pub velocity_scl: f32,
  pub lifetime_scl: f32,
}

impl Packet for CreateBulletPacket {}

impl TryFrom<&Vec<u8>> for CreateBulletPacket {
  type Error = Error;
  fn try_from(byte_vector: &Vec<u8>) -> Result<Self, Self::Error> {
    let mut byte_buffer = ByteBuffer::from_bytes(byte_vector);
    let bullet_type = byte_buffer.read_u16()?;
    let team = byte_buffer.read_u16()?;
    let x = byte_buffer.read_f32()?;
    let y = byte_buffer.read_f32()?;
    let angle = byte_buffer.read_f32()?;
    let damage = byte_buffer.read_f32()?;
    let velocity_scl = byte_buffer.read_f32()?;
    let lifetime_scl = byte_buffer.read_f32()?;
    Ok(CreateBulletPacket { bullet_type, team, x, y, angle, damage, velocity_scl, lifetime_scl })
  }
}

impl Into<Vec<u8>> for CreateBulletPacket {
  fn into(self) -> Vec<u8> {
    let mut byte_buffer = ByteBuffer::new();
    byte_buffer.write_u16(self.bullet_type);
    byte_buffer.write_u16(self.team);
    byte_buffer.write_f32(self.x);
    byte_buffer.write_f32(self.y);
    byte_buffer.write_f32(self.angle);
    byte_buffer.write_f32(self.damage);
    byte_buffer.write_f32(self.velocity_scl);
    byte_buffer.write_f32(self.lifetime_scl);
    byte_buffer.into_vec()
  }
}
