use std::{ convert::TryFrom, io::Error };
use crate::contract::registries::Registries

pub use super::packets::admin_request_packet::*;
pub use super::packets::announce_packet::*;
pub use super::packets::assembler_drone_spawned_packet::*;
pub use super::packets::assembler_unit_spawned_packet::*;
pub use super::packets::auto_door_toggle_packet::*;
pub use super::packets::stream_begin::*;
pub use super::packets::stream_chunk::*;
pub use super::packets::connect_packet::*;
pub use super::packets::client_snapshot_packet::*;
pub use super::packets::warning_toast_packet::*;
pub use super::packets::update_gameover_packet::*;

mod admin_request_packet;
mod announce_packet;
mod assembler_drone_spawned_packet;
mod assembler_unit_spawned_packet;
mod auto_door_toggle_packet;
mod stream_begin;
mod connect_packet;
mod stream_chunk;
mod client_snapshot_packet;
mod world_data_begin;
mod warning_toast_packet;
mod update_gameover_packet;

pub trait Packet: for<'f> TryFrom<&'f Vec<u8>, Error = Error> + Into<Vec<u8>> + Default {
  fn from(context: Registries) {
    todo!()
  }
}
