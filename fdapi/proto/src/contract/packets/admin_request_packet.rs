use std::{convert::TryFrom, io::{Error, ErrorKind}};

use bytebuffer::ByteBuffer;
use crate::contract::packets::Packet;
use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Debug, Default, FromPrimitive, ToPrimitive)]
pub enum AdminActions {
    #[default]
    Kick,
    Ban,
    Trace,
    Wave
}

#[derive(Debug, Default)]
pub struct AdminRequestPacket {
    player_id: u16,
    action: AdminActions,
}

impl Packet for AdminRequestPacket {}

impl TryFrom<&Vec<u8>> for AdminRequestPacket {
    type Error = Error;
    fn try_from(byte_vector: &Vec<u8>) -> Result<Self, Self::Error> {
        let mut byte_buffer = ByteBuffer::from_bytes(byte_vector);
        let player_id = byte_buffer.read_u16()?;
        let action = num_traits::FromPrimitive::from_u8(byte_buffer.read_u8()?).ok_or(Error::new(ErrorKind::NotFound, "Invalid variant"))?;
        Ok(AdminRequestPacket { player_id, action })
    }
}

impl Into<Vec<u8>> for AdminRequestPacket {
    fn into(self) -> Vec<u8> {
        let mut byte_buffer = ByteBuffer::new();
        byte_buffer.write_u16(self.player_id);
        byte_buffer.write_u8(); //num_traits::ToPrimitive::to_u8(&self.action).ok_or(Error::new(ErrorKind::NotFound, "Err"))?; не работает
        byte_buffer.into_vec()
    }
}
