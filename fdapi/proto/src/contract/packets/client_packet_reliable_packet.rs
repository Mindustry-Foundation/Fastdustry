use std::{ convert::TryFrom, io::Error };
use bytebuffer::ByteBuffer;
use super::Packet;

#[derive(Debug, Default)]
pub struct ClientPacketReliablePacket {
  packet_type: String,
  data: String,
}

impl Packet for ClientPacketReliablePacket {}

impl TryFrom<&Vec<u8>> for ClientPacketReliablePacket {
  type Error = Error;
  fn try_from(byte_vector: &Vec<u8>) -> Result<Self, Self::Error> {
    let mut byte_buffer = ByteBuffer::from_bytes(byte_vector);
    let packet_type = byte_buffer.read_string()?;
    let data = byte_buffer.read_string()?;
    Ok(ClientPacketReliablePacket { packet_type, data })
  }
}

impl Into<Vec<u8>> for ClientPacketReliablePacket {
  fn into(self) -> Vec<u8> {
    let mut byte_buffer = ByteBuffer::new();
    byte_buffer.write_string(&self.packet_type);
    byte_buffer.write_string(&self.data);
    byte_buffer.into_vec()
  }
}
