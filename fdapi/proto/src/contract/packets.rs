use std::{io::Error, convert::TryFrom};
use base64::{Engine, engine::general_purpose};
use bytebuffer::ByteBuffer;
use unique_type_id::UniqueTypeId;

use crate::TypeIO;

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
    let version_type = byte_buffer.read_short_string()?;
    let name = byte_buffer.read_short_string()?;
    let locale = byte_buffer.read_short_string()?;
    let usid = byte_buffer.read_short_string()?;
    let uuid_bytes = byte_buffer.read_bytes(16)?;
    let uuid = Engine::encode(&general_purpose::STANDARD, uuid_bytes);
    let mobile = byte_buffer.read_u8()? == 1;
    let color = byte_buffer.read_u32()?;
    let totalMods = byte_buffer.read_u8()?;
    let mut mods = Vec::<String>::new();
    for i in 0..totalMods {
      mods.push(byte_buffer.read_short_string()?);
    }
    Ok(ConnectPacket {
      version, version_type, name, locale, usid, uuid, mobile, color, mods
    })
  }
}

impl Into<Vec<u8>> for ConnectPacket {
  fn into(self) -> Vec<u8> {
    let mut byte_buffer = ByteBuffer::new();
    byte_buffer.write_u32(self.version);
    byte_buffer.write_short_string(&self.version_type);
    byte_buffer.write_short_string(&self.name);
    byte_buffer.write_short_string(&self.locale);
    byte_buffer.write_short_string(&self.usid);
    byte_buffer.write_bytes(&Engine::decode(&general_purpose::STANDARD, self.uuid).unwrap());
    byte_buffer.write_u8(if self.mobile {1} else {0});
    byte_buffer.write_u32(self.color);
    for i in self.mods {
      byte_buffer.write_short_string(&i);
    }
    byte_buffer.into_vec()
  }
}