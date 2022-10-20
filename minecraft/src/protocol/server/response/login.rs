use uuid::Uuid;

use crate::{protocol::server::packet::SerializePacket, util::binary::PacketWriter, chat::message::Message};

pub struct EncryptionPacket {

}

pub enum LoginPacket {
    Success(Uuid, String), Block(Message)
}

impl SerializePacket for LoginPacket {
    fn serialize(&self) -> Vec<u8> {
        let mut writer = PacketWriter::new();
        match self {
            Self::Success(uuid, name) => {
                writer.write(0x02 as u8);
                writer.write(uuid.to_string());
                writer.write_varint(0);
            }
            Self::Block(reason) => {
                writer.write(0x00 as u8);
                writer.write(reason);
            }
        }

        writer.bytes()
    }
}