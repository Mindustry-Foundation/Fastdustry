use std::io::{ Write, Result };

use super::packets::{ Packet, StreamBegin };

pub struct StreamPacket<T: Packet> {
  begin: StreamBegin,
  packet: T,
  buffer: Vec<u8>,
}

impl<T: Packet> StreamPacket<T> {
  fn new(begin: StreamBegin) -> Self {
    StreamPacket {
      begin,
      packet: T::default(),
      buffer: Vec::new(),
    }
  }
}

impl<T: Packet> Write for StreamPacket<T> {
  fn write(&mut self, buf: &[u8]) -> Result<usize> {
    self.buffer.write(buf)
  }

  fn flush(&mut self) -> Result<()> {
    self.buffer.flush()
  }
}
