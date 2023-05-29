use std::{ convert::{ TryFrom, TryInto }, io::{ Error, ErrorKind }, ops::Range };

use base64::{ Engine, engine::general_purpose };
use bytebuffer::ByteBuffer;

use crate::{ contract::packets::Packet, ReadStruct, WriteStruct };

#[derive(Debug, Default)]
pub struct ConnectPacket {
  pub version: u32,
  pub version_type: String,
  pub name: String,
  pub locale: String,
  pub usid: String,
  pub uuid: [u8; 16],
  pub mobile: bool,
  pub color: u32,
  pub mods: Vec<String>,
}

impl Packet for ConnectPacket {}

impl TryFrom<&Vec<u8>> for ConnectPacket {
  type Error = Error;

  fn try_from(byte_vector: &Vec<u8>) -> Result<Self, Self::Error> {
    let mut byte_buffer = ByteBuffer::from_bytes(byte_vector);
    let mut mods = Vec::<String>::new();

    let version = byte_buffer.read_u32()?;
    let version_type = byte_buffer.read_struct()?;
    let name = byte_buffer.read_struct()?;
    let locale = byte_buffer.read_struct()?;
    let usid = byte_buffer.read_struct()?;
    let uuid: [u8; 16] = byte_buffer
      .read_bytes(16)?
      .try_into()
      .map_err(|_| Error::new(ErrorKind::InvalidData, "Invalid UUID size"))?;
    let mobile = byte_buffer.read_u8()? == 1;
    let color = byte_buffer.read_u32()?;
    let total_mods = byte_buffer.read_u8()?;

    (Range {
      start: 0,
      end: total_mods,
    })
      .map(
        |_| -> Result<(), Error> {
          mods.push(byte_buffer.read_struct()?);
          Ok(())
        }
      )
      .collect::<Result<(), Error>>()?;

    Ok(ConnectPacket {
      version,
      version_type,
      name,
      locale,
      usid,
      uuid,
      mobile,
      color,
      mods,
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
    byte_buffer.write_u8(if self.mobile { 1 } else { 0 });
    byte_buffer.write_u32(self.color);

    for i in self.mods {
      byte_buffer.write_struct(&i);
    }

    byte_buffer.into_vec()
  }
}
