use bytebuffer::ByteBuffer;
use unique_type_id::UniqueTypeId;

#[derive(Debug)]
pub enum PacketType {
  StreamBegin(StreamBegin),
  StreamChunk(StreamChunk),
  WorldStream(WorldStream),
  ConnectPacket(ConnectPacket),
  UnhandledPacket
}

#[derive(UniqueTypeId, Debug)]
#[UniqueTypeIdFile = "fdapi/proto/packets.toml"]
pub struct StreamBegin {
  pub id: u32,
  pub total: u32,
  pub stream_type: u8
}

#[derive(UniqueTypeId, Debug)]
#[UniqueTypeIdFile = "fdapi/proto/packets.toml"]
pub struct StreamChunk {}

#[derive(UniqueTypeId, Debug)]
#[UniqueTypeIdFile = "fdapi/proto/packets.toml"]
pub struct WorldStream {}

#[derive(UniqueTypeId, Debug)]
#[UniqueTypeIdFile = "fdapi/proto/packets.toml"]
pub struct ConnectPacket {}

impl Packet for PacketType {}

impl From<&Vec<u8>> for PacketType {
  fn from(byte_buffer: &Vec<u8>) -> Self {
    let mut byte_buffer = ByteBuffer::from_bytes(byte_buffer);

    match byte_buffer.read_u8() {
      Ok(packet_id) => match packet_id {
        packet_id if packet_id == StreamBegin::id().0 as u8 => match (
          byte_buffer.read_u32(),
          byte_buffer.read_u32(),
          byte_buffer.read_u8()
        ) {
          (Ok(id), Ok(total), Ok(stream_type)) => Self::StreamBegin(StreamBegin {
            id,
            total,
            stream_type
          }),
          (_v1, _v2, _v3) => Self::UnhandledPacket
        },
        _ => Self::UnhandledPacket
      },
      Err(_) => Self::UnhandledPacket
    }
  }
}

impl Into<Vec<u8>> for PacketType {
  fn into(self) -> Vec<u8> {
    let mut byte_buffer = ByteBuffer::new();

    match self {
      Self::StreamBegin(stream_begin) => {
        byte_buffer.write_u32(stream_begin.id);
        byte_buffer.write_u32(stream_begin.total);
        byte_buffer.write_u8(stream_begin.stream_type);

        byte_buffer.into_vec()
      }
      _ => vec![]
    }
  }
}

trait Packet : for<'f> From<&'f Vec<u8>> + Into<Vec<u8>> {}