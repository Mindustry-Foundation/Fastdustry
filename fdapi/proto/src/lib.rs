use std::io::{Error, ErrorKind};

use bytebuffer::ByteBuffer;
use cgmath::Vector2;
use content::tile::Tile;

#[macro_use]
extern crate unique_type_id_derive;
extern crate unique_type_id;
extern crate bytebuffer;
extern crate content;
extern crate base64;
extern crate cgmath;

impl WriteStruct<&Vector2<u32>> for ByteBuffer {
  fn write_struct(&mut self, val: &Vector2<u32>) {
    self.write_u32(val.x);
    self.write_u32(val.y);
  }
}

impl WriteStruct<&Vector2<f32>> for ByteBuffer {
  fn write_struct(&mut self, val: &Vector2<f32>) {
    self.write_f32(val.x);
    self.write_f32(val.y);
  }
}

impl WriteStruct<&String> for ByteBuffer {
  fn write_struct(&mut self, val: &String) {
    self.write_u16(val.len() as u16);
    self.write_bytes(val.as_bytes());
  }
}

impl WriteStruct<&Tile> for ByteBuffer {
  fn write_struct(&mut self, val: &Tile) {
    self.write_struct(&val.pos)
  }
}

impl ReadStruct<Vector2<u32>> for ByteBuffer {
  fn read_struct(&mut self) -> Result<Vector2<u32>, Error> {
    Ok(Vector2::new(
      self.read_u32()?,
      self.read_u32()?
    ))
  }
}

impl ReadStruct<Vector2<f32>> for ByteBuffer {
  fn read_struct(&mut self) -> Result<Vector2<f32>, Error> {
    Ok(Vector2::new(
      self.read_f32()?,
      self.read_f32()?
    ))
  }
}

impl ReadStruct<String> for ByteBuffer {
  fn read_struct(&mut self) -> Result<String, Error> {
    let size = self.read_u16()?;

    match String::from_utf8(self.read_bytes(size as usize)?) {
      Ok(string_result) => Ok(string_result),
      Err(e) => Err(Error::new(ErrorKind::InvalidData, e)),
    }
  }
}

impl ReadStruct<Tile> for ByteBuffer {
  fn read_struct(&mut self) -> Result<Tile, Error> {
    Ok(Tile {
      pos: self.read_struct()?
    })
  }
}

trait WriteStruct<S> {
  fn write_struct(&mut self, val: S);
}

trait ReadStruct<S> {
  fn read_struct(&mut self) -> Result<S, Error>;
}

pub mod contract;