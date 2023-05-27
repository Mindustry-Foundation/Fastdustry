use bytebuffer::ByteBuffer;
use proto::contract::packets::ClientSnapshotPacket;

fn main() {
  let raw: Vec<i8> = vec![
    0,
    0,
    0,
    4,
    0,
    0,
    0,
    4,
    0,
    64,
    -128,
    0,
    0,
    64,
    -128,
    0,
    0,
    64,
    -128,
    0,
    0,
    64,
    -128,
    0,
    0,
    64,
    -128,
    0,
    0,
    64,
    -128,
    0,
    0,
    64,
    -128,
    0,
    0,
    64,
    -128,
    0,
    0,
    -1,
    -1,
    -1,
    -1,
    0,
    0,
    0,
    0,
    -1,
    -1,
    -1,
    -1,
    64,
    -128,
    0,
    0,
    64,
    -128,
    0,
    0,
    64,
    -128,
    0,
    0,
    64,
    -128,
    0,
    0
  ];
  let data: Vec<u8> = raw
    .iter()
    .map(|el| *el as u8)
    .to_owned()
    .collect();
  let mut buf = ByteBuffer::from_vec(data);

  let a = buf.read_u8();

  let c = ClientSnapshotPacket::try_from(&buf.into_vec()).unwrap();

  println!("Hello, world! {:?}", c);
}
