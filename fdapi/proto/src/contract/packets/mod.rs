use std::{ convert::TryFrom, io::Error };

pub use super::packets::stream_begin::*;
pub use super::packets::stream_chunk::*;
pub use super::packets::connect_packet::*;
pub use super::packets::client_snapshot_packet::*;

mod stream_begin;
mod connect_packet;
mod stream_chunk;
mod client_snapshot_packet;

pub trait Packet: for<'f> TryFrom<&'f Vec<u8>, Error = Error> + Into<Vec<u8>> + Default {}
