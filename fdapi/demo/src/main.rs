use proto::contract::packets::{PacketType, StreamBegin};

fn main() {
  let vec: Vec<u8> = PacketType::into(PacketType::StreamBegin(StreamBegin {
    id: 1,
    total: 213123,
    stream_type: 3
  }));

  let packet = PacketType::from(&vec);
  
  println!("Hello, world! {:?}", packet);
}
