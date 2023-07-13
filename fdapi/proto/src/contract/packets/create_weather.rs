use std::{ convert::TryFrom, io::Error };
use bytebuffer::ByteBuffer;
use super::Packet;

#[derive(Debug, Default)]
pub struct CreateWeather {
  pub weather: u16, //Weather !todo
  pub intensivity: f32,
  pub duration: f32,
  pub wind_x: f32,
  pub wind_y: f32,
}

impl Packet for CreateWeather {}

impl TryFrom<&Vec<u8>> for CreateWeather {
  type Error = Error;
  fn try_from(byte_vector: &Vec<u8>) -> Result<Self, Self::Error> {
    let mut byte_buffer = ByteBuffer::from_bytes(byte_vector);
    let weather = byte_buffer.read_u16()?;
    let intensivity = byte_buffer.read_f32()?;
    let duration = byte_buffer.read_f32()?;
    let wind_x = byte_buffer.read_f32()?;
    let wind_y = byte_buffer.read_f32()?;
    Ok(CreateWeather { weather, intensivity, duration, wind_x, wind_y })
  }
}

impl Into<Vec<u8>> for CreateWeather {
  fn into(self) -> Vec<u8> {
    let mut byte_buffer = ByteBuffer::new();
    byte_buffer.write_u16(self.weather);
    byte_buffer.write_f32(self.intensivity);
    byte_buffer.write_f32(self.duration);
    byte_buffer.write_f32(self.wind_x);
    byte_buffer.write_f32(self.wind_y);
    byte_buffer.into_vec()
  }
}
