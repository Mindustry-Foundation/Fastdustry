use std::io::{Error, ErrorKind};

use bytebuffer::ByteBuffer;

extern crate multi_default_trait_impl;
#[macro_use]
extern crate unique_type_id_derive;
extern crate unique_type_id;
extern crate bytebuffer;
extern crate base64;

impl TypeIO for ByteBuffer {
  fn write_short_string(&mut self, val: &str) {
    self.write_u16(val.len() as u16);
    self.write_bytes(val.as_bytes());
  }

  fn read_short_string(&mut self) -> Result<String, Error> {
    let size = self.read_u16()?;

    match String::from_utf8(self.read_bytes(size as usize)?) {
        Ok(string_result) => Ok(string_result),
        Err(e) => Err(Error::new(ErrorKind::InvalidData, e)),
    }
  }
}

trait TypeIO {
  fn write_short_string(&mut self, val: &str);

  fn read_short_string(&mut self) -> Result<String, Error>;
}

pub mod contract;