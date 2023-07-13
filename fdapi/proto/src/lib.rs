use std::io::{ Error, ErrorKind };

use bytebuffer::ByteBuffer;
// use content::{ team::{ Team, TEAMS }, unit::{ UNIT_TYPES, UnitType }, tile::Tile };
use vectora::types::vector::Vector2d;

extern crate bytebuffer;
extern crate content;
extern crate base64;
extern crate vectora;
extern crate num_derive;
extern crate num_traits;

impl WriteStruct<&Vector2d<u32>> for ByteBuffer {
  fn write_struct(&mut self, val: &Vector2d<u32>) {
    self.write_u32(*val.components.get(0).unwrap_or(&0));
    self.write_u32(*val.components.get(1).unwrap_or(&0));
  }
}

impl WriteStruct<&Vector2d<i32>> for ByteBuffer {
  fn write_struct(&mut self, val: &Vector2d<i32>) {
    self.write_i32(*val.components.get(0).unwrap_or(&0));
    self.write_i32(*val.components.get(1).unwrap_or(&0));
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

impl WriteStruct<&Team> for ByteBuffer {
  fn write_struct(&mut self, val: &Team) {
    self.write_u16(val.id())
  }
}

impl WriteStruct<&UnitType> for ByteBuffer {
  fn write_struct(&mut self, val: &UnitType) {
    self.write_u16(val.id())
  }
}

impl ReadStruct<Vector2d<u32>> for ByteBuffer {
  fn read_struct(&mut self) -> Result<Vector2d<u32>, Error> {
    Ok(Vector2d::from([self.read_u32()?, self.read_u32()?]))
  }
}

impl ReadStruct<Vector2d<i32>> for ByteBuffer {
  fn read_struct(&mut self) -> Result<Vector2d<i32>, Error> {
    Ok(Vector2d::from([self.read_i32()?, self.read_i32()?]))
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

impl<'a> ReadStruct<&'a Team> for ByteBuffer {
  fn read_struct(&mut self) -> Result<&'a Team, Error> {
    let team_id = self.read_u8()?;

    // TEAMS.lock()
    //   .unwrap()
    //   .get(team_id as usize)
    //   .ok_or(
    //     Error::new(ErrorKind::NotFound, format!("team with id {id} does not exist", id = team_id))
    //   )
    todo!()
  }
}

impl<'a> ReadStruct<&'a UnitType> for ByteBuffer {
  fn read_struct(&mut self) -> Result<&'a UnitType, Error> {
    let unit_id = self.read_u8()?;

    // UNIT_TYPES.lock()
    //   .unwrap()
    //   .get(unit_id as usize)
    //   .ok_or(
    //     Error::new(ErrorKind::NotFound, format!("unit with id {id} does not exist", id = unit_id))
    //   )
    todo!()
  }
}

trait WriteStruct<S> {
  fn write_struct(&mut self, val: S);
}

trait ReadStruct<S> {
  fn read_struct(&mut self) -> Result<S, Error>;
}

pub mod contract;
