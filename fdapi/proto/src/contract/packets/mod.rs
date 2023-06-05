use std::{ convert::TryFrom, io::Error };

pub use super::packets::admin_request_packet::*;
pub use super::packets::announce_packet::*;
pub use super::packets::assembler_drone_spawned_packet::*;
pub use super::packets::assembler_unit_spawned_packet::*;
pub use super::packets::auto_door_toggle_packet::*;
pub use super::packets::begin_break_packet::*;
pub use super::packets::begin_place_packet::*;
pub use super::packets::block_snapshot_packet::*;
pub use super::packets::build_destroyed_packet::*;
pub use super::packets::buildings_health_update_packet::*;
pub use super::packets::building_control_select_packet::*;
pub use super::packets::clear_items_packet::*;
pub use super::packets::client_packet_reliable_packet::*;
pub use super::packets::client_packet_unreliable_packet::*;
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
mod begin_break_packet;
mod begin_place_packet;
mod block_snapshot_packet;
mod build_destroyed_packet;
mod buildings_health_update_packet;
mod building_control_select_packet;
mod clear_items_packet;
mod client_packet_reliable_packet;
mod client_packet_unreliable_packet;
mod stream_begin;
mod connect_packet;
mod stream_chunk;
mod client_snapshot_packet;
mod world_data_begin;
mod warning_toast_packet;
mod update_gameover_packet;
pub mod packets;

pub trait Packet: for<'f> TryFrom<&'f Vec<u8>, Error = Error> + Into<Vec<u8>> + Default {}
