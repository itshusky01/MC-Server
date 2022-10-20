use crate::error::{Result, Error, ErrorKind};
use crate::util::binary::*;
use crate::protocol::server::packet::{Packet, DedeserializePacket};

pub struct HandshakePacket {
    pub version: i32,
    pub address: String,
    pub port: u16,
    pub status: HandshakeStatus
}

impl std::fmt::Debug for HandshakePacket {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Handshake(Version: {}, Address: {}, Port: {}, Status: {})", 
            self.version, self.address, self.port, {if let HandshakeStatus::Status = self.status { "Status" } else { "Login" } })
    }
}

pub enum HandshakeStatus {
    Status, Login
}

impl DedeserializePacket for HandshakePacket {
    fn deserialize(packet: Packet) -> Result<Self> {
        if  packet.id != 0x00 {
            return Err(Error::new(ErrorKind::BrokenPacket, "Broken Packet"))
        }

        let mut reader = BinaryReader::new(packet.data.clone());
        let version;
        let address;
        let port;
        let status;

        match reader.read_varint() {
            Err(e) => return Err(e),
            Ok(v) => version = v
        }

        match reader.read_string() {
            Err(e) => return Err(e),
            Ok(v) => address = v
        }

        match reader.read::<u16>() {
            Err(e) => return Err(e),
            Ok(v) => port = v
        }

        match reader.read::<u8>() {
            Err(e) => return Err(e),
            Ok(v) => {
                match v {
                    0x01 => status = HandshakeStatus::Status,
                    0x02 => status = HandshakeStatus::Login,
                    _ => return Err(Error::new(ErrorKind::BrokenPacket, "Broken Handshake Packet"))
                }
            }
        }

        Ok(Self { version, address, port, status })
    }
}