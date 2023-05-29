use std::io::{ Result, Write };

use crate::contract::packets::StreamBegin;

#[derive(Debug, Default)]
pub struct StreamPacket {
  pub begin: StreamBegin,
  pub buffer: Vec<u8>,
}

impl StreamPacket {
  pub fn new(begin: StreamBegin, buffer: Vec<u8>) -> Self {
    Self { begin, buffer }
  }
}

impl Write for StreamPacket {
  fn write(&mut self, buf: &[u8]) -> Result<usize> {
    self.buffer.write(buf)
  }

  fn flush(&mut self) -> Result<()> {
    self.buffer.flush()
  }
}
