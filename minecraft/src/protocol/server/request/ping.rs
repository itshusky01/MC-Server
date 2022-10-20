use crate::error::{Error, ErrorKind, Result};
use crate::{util::binary::BinaryReader, protocol::server::packet::DedeserializePacket};

pub enum PingPacket {
    Ping { payload: i64 },
    ServerListPing()
}

impl DedeserializePacket for PingPacket {
    fn deserialize(packet: crate::protocol::server::packet::Packet) -> Result<Self> {
        match packet.id {
            0x00 => Ok(Self::ServerListPing()),
            0x01 => {
              match BinaryReader::new(packet.data).read::<i64>() {
                Ok(v) => Ok(Self::Ping { payload: v }),
                Err(err) => Err(err),
            }  
            },
            _ => Err(Error::new(ErrorKind::BrokenPacket, "Broken Packet"))
        }
    }
}