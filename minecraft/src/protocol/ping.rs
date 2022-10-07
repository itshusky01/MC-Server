
use std::io::Result;

use serde::Serialize;

use crate::util::writer::PacketWriter;

use super::response::PacketSerializable;

#[derive(Serialize, Debug)]
pub struct PlayerSample {
    name: String,
    id: String
}

#[derive(Serialize, Debug)]
pub struct ServerListPingPlayers {
    max: i32,
    online: i32,
    sample: Vec<PlayerSample>
}

#[derive(Serialize, Debug)]
pub struct ServerListPingVersion {
    name: String,
    protocol: i32
}

#[derive(Serialize, Debug)]
pub struct ServerListPing {
    version: ServerListPingVersion,
    players: ServerListPingPlayers,
    description: String,
    favicon: String
}

pub enum ResponsePacket {
    Pong { payload: i64 }
}

impl PacketSerializable for ResponsePacket {
    fn serialize(&self) -> Vec<u8> {
        let mut writer = PacketWriter::new();

        match self {
            ResponsePacket::Pong { payload } => {
                writer.write(0x01 as u8);
                writer.write(*payload);
            },
        }

        writer.bytes()
    }
}