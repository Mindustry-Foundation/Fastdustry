use bytebuffer::ByteBuffer;
use unique_type_id::{UniqueTypeId};

enum PacketType {
  StreamBegin(StreamBegin),
  UnhandledPacket
}

#[derive(UniqueTypeId)]
#[UniqueTypeIdFile = "proto/packets.toml"]
struct StreamBegin {}

impl Packet for PacketType {}

impl From<&Vec<u8>> for PacketType {
  fn from(byte_buffer: &Vec<u8>) -> Self {
    let mut byte_buffer = ByteBuffer::from_bytes(byte_buffer);

    match byte_buffer.read_u8() {
      Ok(packet_id) => match packet_id {
        packet_id if packet_id == StreamBegin::id().0 as u8 =>
          todo!(),
        _ => PacketType::UnhandledPacket
      },
      Err(_) => PacketType::UnhandledPacket
    }
  }
}

impl Into<Vec<u8>> for PacketType {
  fn into(self) -> Vec<u8> {
    todo!()
  }
}

fn main() {
  let stream_begin: Vec<u8> = PacketType::StreamBegin(StreamBegin {  }).into();
}

trait Packet : for<'f> From<&'f Vec<u8>> + Into<Vec<u8>> {}