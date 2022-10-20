use crate::error::{Error, ErrorKind, Result};
use crate::protocol::server::packet::Packet;
use crate::{util::binary::BinaryReader, protocol::server::packet::DedeserializePacket};

pub struct SigData {
    timestamp: u64,
    pub_key: Vec<u8>,
    signature: Vec<u8>
}

pub struct LoginStartPacket {
    name: String,
    sid_data: Option<SigData>,
    uuid: Option<uuid::Uuid>
}

impl DedeserializePacket for LoginStartPacket {
    fn deserialize(packet: Packet) -> Result<Self> {
        if packet.id != 0x00 {
            return Err(Error::new(ErrorKind::BrokenPacket, "Broken Packet"));
        }

        let mut reader = BinaryReader::new(packet.data);

        let name;
        match reader.read_string() {
            Ok(n) => name = n,
            Err(e) => return Err(e)
        }

        Ok(LoginStartPacket { name: name, sid_data: None, uuid: None })
    }
}