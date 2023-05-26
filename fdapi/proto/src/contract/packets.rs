use std::{io::Error, convert::TryFrom, collections::VecDeque};
use base64::{Engine, engine::general_purpose};
use bytebuffer::ByteBuffer;
use cgmath::Vector2;
use content::block::BlockPlan;
use unique_type_id::UniqueTypeId;

use crate::{ReadStruct, WriteStruct};

#[derive(UniqueTypeId, Debug)]
#[UniqueTypeIdFile = "fdapi/proto/packets.toml"]
pub struct StreamBegin {
  pub id: u32,
  pub total: u32,
  pub stream_type: u8
}

impl Packet for StreamBegin {
  fn empty() -> Self {
    StreamBegin {
      id: 0,
      total: 0,
      stream_type: 0
    }
  }
}

impl TryFrom<&Vec<u8>> for StreamBegin {
  type Error = Error;

  fn try_from(byte_vector: &Vec<u8>) -> Result<Self, Self::Error> {
    let mut byte_buffer = ByteBuffer::from_bytes(byte_vector);
      let id = byte_buffer.read_u32()?;
      let total = byte_buffer.read_u32()?;
      let stream_type = byte_buffer.read_u8()?;
      Ok(
        StreamBegin { id, total, stream_type }
      )
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

pub trait Packet : for<'f> TryFrom<&'f Vec<u8>, Error = Error> + Into<Vec<u8>> {
  fn empty() -> Self;
}

#[derive(UniqueTypeId, Debug)]
#[UniqueTypeIdFile = "fdapi/proto/packets.toml"]
pub struct StreamChunk {
  pub id: u32,
  pub data: Vec<u8>
}

impl Packet for StreamChunk {
  fn empty() -> Self {
    StreamChunk {
      id: 0,
      data: Vec::new()
    }
  }
}

impl TryFrom<&Vec<u8>> for StreamChunk {
  type Error = Error;

  fn try_from(byte_vector: &Vec<u8>) -> Result<Self, Self::Error> {
    let mut byte_buffer = ByteBuffer::from_bytes(byte_vector);
    let id = byte_buffer.read_u32()?;
    let data = byte_buffer.read_u32()
      .map(|length| byte_buffer.read_bytes(length as usize)).unwrap()?;

    Ok(StreamChunk {
      id,
      data
    })
  }
}

impl Into<Vec<u8>> for StreamChunk {
  fn into(self) -> Vec<u8> {
    let mut byte_buffer = ByteBuffer::new();
    byte_buffer.write_u32(self.id);
    byte_buffer.write_bytes(&self.data);
    byte_buffer.into_vec()
  }
}

#[derive(UniqueTypeId, Debug)]
#[UniqueTypeIdFile = "fdapi/proto/packets.toml"]
pub struct ConnectPacket {
  pub version: u32,
  pub version_type: String,
  pub name: String,
  pub locale: String,
  pub usid: String,
  pub uuid: String,
  pub mobile: bool,
  pub color: u32,
  pub mods: Vec<String>
}

impl Packet for ConnectPacket {
  fn empty() -> Self {
    ConnectPacket {
      version: 0,
      version_type: String::new(),
      name: String::new(),
      locale: String::new(),
      usid: String::new(),
      uuid: String::new(),
      mobile: false,
      color: 0,
      mods: Vec::new()
    }
  }
}

impl TryFrom<&Vec<u8>> for ConnectPacket {
  type Error = Error;

  fn try_from(byte_vector: &Vec<u8>) -> Result<Self, Self::Error> {
    let mut byte_buffer = ByteBuffer::from_bytes(byte_vector);

    let version = byte_buffer.read_u32()?;
    let version_type = byte_buffer.read_struct()?;
    let name = byte_buffer.read_struct()?;
    let locale = byte_buffer.read_struct()?;
    let usid = byte_buffer.read_struct()?;
    let uuid_bytes = byte_buffer.read_bytes(16)?;
    let uuid = Engine::encode(&general_purpose::STANDARD, uuid_bytes);
    let mobile = byte_buffer.read_u8()? == 1;
    let color = byte_buffer.read_u32()?;
    let total_mods = byte_buffer.read_u8()?;
    let mut mods = Vec::<String>::new();

    for _i in 0..total_mods {
      mods.push(byte_buffer.read_struct()?);
    }

    Ok(ConnectPacket {
      version,
      version_type,
      name, locale,
      usid,
      uuid,
      mobile,
      color,
      mods
    })
  }
}

impl Into<Vec<u8>> for ConnectPacket {
  fn into(self) -> Vec<u8> {
    let mut byte_buffer = ByteBuffer::new();
    byte_buffer.write_u32(self.version);
    byte_buffer.write_struct(&self.version_type);
    byte_buffer.write_struct(&self.name);
    byte_buffer.write_struct(&self.locale);
    byte_buffer.write_struct(&self.usid);
    byte_buffer.write_bytes(&Engine::decode(&general_purpose::STANDARD, self.uuid).unwrap());
    byte_buffer.write_u8(if self.mobile {1} else {0});
    byte_buffer.write_u32(self.color);

    for i in self.mods {
      byte_buffer.write_struct(&i);
    }

    byte_buffer.into_vec()
  }
}

#[derive(Debug)]
pub struct ClientSnapshotPacket<'a> {
  snapshot_id: u32,
  mining_pos: i32,
  unit_id: u32,
  pos: Vector2<f32>,
  mouse_pos: Vector2<f32>,
  view_pos: Vector2<f32>,
  view_size: Vector2<f32>,
  velocity: Vector2<f32>,
  rotation: f32,
  base_rotation: f32,
  plans: VecDeque<BlockPlan<'a>>,
  dead: bool,
  boosting: bool,
  shooting: bool,
  chatting: bool,
  building: bool
}

impl Packet for ClientSnapshotPacket<'_> {
  fn empty() -> Self {
    ClientSnapshotPacket {
      snapshot_id: 0,
      mining_pos: 0,
      unit_id: 0,
      pos: Vector2::new(0.0, 0.0),
      mouse_pos: Vector2::new(0.0, 0.0),
      view_pos: Vector2::new(0.0, 0.0),
      view_size: Vector2::new(0.0, 0.0),
      velocity: Vector2::new(0.0, 0.0),
      rotation: 0.0,
      base_rotation: 0.0,
      plans: VecDeque::new(),
      dead: false,
      boosting: false,
      shooting: false,
      chatting: false,
      building: false
    }
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
      view_size: byte_buffer.read_struct()?
    })
  }
}