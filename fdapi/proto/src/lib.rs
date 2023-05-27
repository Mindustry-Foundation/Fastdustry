use std::io::{ Error, ErrorKind };

use bytebuffer::ByteBuffer;
use content::tile::Tile;
use vectora::types::vector::Vector2d;

extern crate bytebuffer;
extern crate content;
extern crate base64;
extern crate vectora;

impl WriteStruct<&Vector2d<u32>> for ByteBuffer {
  fn write_struct(&mut self, val: &Vector2d<u32>) {
    self.write_u32(*val.components.get(0).unwrap_or(&0));
    self.write_u32(*val.components.get(1).unwrap_or(&0));
  }
}

impl WriteStruct<&Vector2d<f32>> for ByteBuffer {
  fn write_struct(&mut self, val: &Vector2d<f32>) {
    self.write_f32(*val.components.get(0).unwrap_or(&0.0));
    self.write_f32(*val.components.get(1).unwrap_or(&0.0));
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

impl ReadStruct<Vector2d<u32>> for ByteBuffer {
  fn read_struct(&mut self) -> Result<Vector2d<u32>, Error> {
    Ok(Vector2d::from([self.read_u32()?, self.read_u32()?]))
  }
}

impl ReadStruct<Vector2d<f32>> for ByteBuffer {
  fn read_struct(&mut self) -> Result<Vector2d<f32>, Error> {
    Ok(Vector2d::from([self.read_f32()?, self.read_f32()?]))
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
      pos: self.read_struct()?,
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
