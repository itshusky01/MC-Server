use std::io::{Result, Error, ErrorKind};
use log::{info, warn};

use crate::util::reader::BinaryReader;

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
            Ok(v) => {
                if v == 254 {
                    // pass
                    return Err(Error::new(ErrorKind::Other, "Legacy ping, not supported"));
                }

                length = v as usize;
            }
        }
        
        match reader.read_varint() {
            Err(e) => return Err(e),
            Ok(v) => id = v
        }

        match reader.read_bytes(length - 1) {
            Err(e) => return Err(e),
            Ok(v) => data = v,
        }

        Ok(Self {
            length,
            id,
            data
        })
    }

    pub fn clone(&self) -> Self {
        Self { length: self.length, id: self.id, data: self.data.clone() }
    }
}