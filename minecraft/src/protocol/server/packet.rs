use crate::error::{Result, Error, ErrorKind};
use crate::util::binary::*;

#[derive(Clone)]
pub struct Packet {
    pub length: usize,
    pub id: i32,
    pub data: Vec<u8>
}

impl Packet {
    pub fn parse(bytes: Vec<u8>) -> Result<Self> {
        let mut reader = BinaryReader::new(bytes);
        let length;
        let id;
        let data;

        match reader.read_varint() {
            Err(e) => return Err(e),
            Ok(v) => length = v as usize
        }
        
        match reader.read_varint() {
            Err(e) => return Err(e),
            Ok(v) => id = v
        }

        match reader.read_bytes(length - 1) {
            Err(e) => return Err(e),
            Ok(v) => data = v,
        }

        Ok(Self { length, id, data})
    }

    pub fn clone(&self) -> Self {
        Self { length: self.length, id: self.id, data: self.data.clone() }
    }
}

pub trait SerializePacket {
    fn serialize(&self) -> Vec<u8>;
}

pub trait DedeserializePacket {
    fn deserialize(packet: Packet) -> Result<Self> where Self: Sized;
}

pub trait ReadPacket : ReadVarInt + ReadN {
    fn read_packet(&mut self) -> Result<Packet> {
        let len;
        match self.read_varint() {
            Err(e) => return Err(e),
            Ok(v) => len = v
        }

        match len {
            -1 | 0 => return Err(Error::new(ErrorKind::EOF, "EOF")),
            254 => return Err(Error::new(ErrorKind::Deprecated, "LegacyPing")),
            x => {
                if x >= 10240 /*limit*/ {
                    return Err(Error::new(ErrorKind::OutOfRange, "Out of limit"))
                }
            }
        }

        let mut buf;
        match self._read(len as usize) {
            Err(e) => return Err(e),
            Ok(v) => buf = v
        }
        
        let mut _buf = Vec::new();
        _buf.append(&mut VarInt::as_varint(len));
        _buf.append(&mut buf);
        Packet::parse(_buf)
    }
}