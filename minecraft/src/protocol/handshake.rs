
use std::io::{Result, ErrorKind, Error};

use crate::util::reader::BinaryReader;

use super::packet::Packet;

pub struct Handshake {
    pub version: i32,
    pub address: String,
    pub port: u16,
    pub status: HandshakeStatus
}

impl std::fmt::Debug for Handshake {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Handshake(Version: {}, Address: {}, Port: {}, Status: {})", 
            self.version, self.address, self.port, {if let HandshakeStatus::Status = self.status { "Status" } else { "Login" } })
    }
}

pub enum HandshakeStatus {
    Status, Login
}

impl Handshake {
    pub fn parse(packet: &Packet) -> Result<Self> {
        if  packet.id != 0x00 {
            return Err(Error::new(ErrorKind::Other, "Not a handshake packet"));
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

        match reader.read_16() {
            Err(e) => return Err(e),
            Ok(v) => port = v as u16
        }

        match reader.read_8() {
            Err(e) => return Err(e),
            Ok(v) => {
                match v {
                    0x01 => status = HandshakeStatus::Status,
                    0x02 => status = HandshakeStatus::Login,
                    _ => return Err(Error::new(ErrorKind::Other, "Seems a broken handshake packet"))
                }
            }
        }

        Ok(Self { version, address, port, status })
    }
}