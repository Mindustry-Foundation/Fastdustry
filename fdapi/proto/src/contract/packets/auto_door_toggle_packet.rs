use std::{convert::TryFrom, io::Error};

use bytebuffer::ByteBuffer;
use content::tile::Tile;

use crate::{ReadStruct, WriteStruct};

use super::Packet;

#[derive(Debug, Default)]
pub struct AutoDoorTogglePacket {
    tile: Tile,
    open: bool
}

impl Packet for AutoDoorTogglePacket {}

impl TryFrom<&Vec<u8>> for AutoDoorTogglePacket {
    type Error = Error;
    fn try_from(byte_vector: &Vec<u8>) -> Result<Self, Self::Error> {
        let mut byte_buffer = ByteBuffer::from_bytes(byte_vector);
        let tile = byte_buffer.read_struct()?;
        let open = byte_buffer.read_u8()? != 0;
        Ok(AutoDoorTogglePacket{ tile, open })
    }
}

impl Into<Vec<u8>> for AutoDoorTogglePacket {
    fn into(self) -> Vec<u8> {
        let mut byte_buffer = ByteBuffer::new();
        byte_buffer.write_struct(&self.tile);
        byte_buffer.write_u8(self.open as u8);
        byte_buffer.into_vec()
    }
}