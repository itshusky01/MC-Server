use crate::{protocol::{server::packet::SerializePacket, common::ServerListPing}, util::binary::PacketWriter};

pub enum PongPacket {
    Pong { payload: i64 },
    ServerListPing { data: ServerListPing }
}

impl SerializePacket for PongPacket {
    fn serialize(&self) -> Vec<u8> {
        let mut writer = PacketWriter::new();
        match self {
            Self::Pong { payload } => {
                writer.write(0x01 as u8);
                writer.write(*payload);
            },
            Self::ServerListPing { data } => {
                writer.write(0x00 as u8);
                writer.write(serde_json::to_string(data).unwrap());
            }
        }

        writer.bytes()
    }
}
