use proto::contract::packets::StreamBegin;

fn main() {
  let vec: Vec<u8> = StreamBegin {
    id: 1,
    total: 213123,
    stream_type: 3
  }.into();
  
  println!("Hello, world! {:?}", vec);
}
